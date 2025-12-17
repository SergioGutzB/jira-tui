// Allow module inception warning (ui/ui.rs is intentional)
#![allow(clippy::module_inception)]

pub mod app;
pub mod events;
pub mod handlers;
pub mod keys;
pub mod tui;
pub mod ui;
pub mod widgets;
