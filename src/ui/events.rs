use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

/// Represents the different types of events the application can handle.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
}

/// Handles the aggregation of terminal events and internal ticks.
#[derive(Debug)]
pub struct EventHandler {
    /// Internal receiver (consumed by the main loop).
    receiver: mpsc::UnboundedReceiver<Event>,
    /// Task handle for the background event listener.
    // We keep the handle to ensure the task isn't dropped prematurely,
    // though in this simple app it lives as long as EventHandler.
    _task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Creates a new EventHandler and spawns a background task to listen for inputs.
    pub fn new(tick_rate_ms: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate_ms);
        let (sender, receiver) = mpsc::unbounded_channel();

        let _task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick_delay => {
                        if sender.send(Event::Tick).is_err() { break; }
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if sender.send(Event::Key(key)).is_err() { break; }
                            }
                            CrosstermEvent::Resize(x, y) => {
                                if sender.send(Event::Resize(x, y)).is_err() { break; }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self { receiver, _task }
    }

    /// Returns the next event. This is async and can be awaited.
    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}
