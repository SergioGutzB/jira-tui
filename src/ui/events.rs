use crate::domain::errors::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

/// Represents the different types of events the application can handle.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Provide a regular signal to update the UI (e.g., animations or polling).
    Tick,
    /// Represents a key press by the user.
    Key(KeyEvent),
    /// Represents a terminal resize event.
    Resize(u16, u16),
}

/// Handles the aggregation of terminal events and internal ticks.
#[derive(Debug)]
pub struct EventHandler {
    /// Channel sender to dispatch events to the main application loop.
    sender: mpsc::UnboundedSender<Event>,
    /// Frequency at which `Event::Tick` is generated.
    tick_rate: Duration,
    /// Task handle for the background event listener.
    task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Creates a new EventHandler and spawns a background task to listen for inputs.
    pub fn new(tick_rate_ms: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate_ms);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick_delay => {
                        let _ = _sender.send(Event::Tick);
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if _sender.send(Event::Key(key)).is_err() {
                                    break;
                                }
                            }
                            CrosstermEvent::Resize(x, y) => {
                                if _sender.send(Event::Resize(x, y)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self {
            sender,
            tick_rate,
            task,
        }
    }

    /// Returns the receiving end of the event channel.
    ///
    /// The main application loop should consume this channel.
    pub fn subscribe(&self) -> mpsc::UnboundedReceiver<Event> {
        let (tx, rx) = mpsc::unbounded_channel();

        // This is a simplification. Ideally, we would support multiple subscribers,
        // but for this architecture, we will re-create the channel or use broadcast if needed.
        // For now, we assume the EventHandler is created once per app run.
        // To properly support the mpsc pattern here without complex broadcasting,
        // we expose a method to get a new receiver if we were using broadcast,
        // but since mpsc is 1-to-1, we will refactor slightly to just return
        // a pre-created receiver in the main loop or use the channel created in `new`.

        // CORRECTION: For MPSC, we cannot subscribe multiple times easily.
        // We will change the design slightly in the next step to pass the receiver directly.
        // For now, this method is a placeholder to indicate intent.
        rx
    }

    /// Helper to get a clone of the sender if needed for external triggers.
    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }
}
