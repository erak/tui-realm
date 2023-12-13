//! ## termion
//!
//! this module contains the adapters for termion

extern crate termion;

mod event;
mod listener;
mod terminal;

// -- export
use crate::terminal::Writer;

pub use listener::TermionInputListener;

use super::{Event, Key, KeyEvent, KeyModifiers};
use crate::tui::backend::TermionBackend;
use crate::tui::{Frame as TuiFrame, Terminal as TuiTerminal};

// -- Frame

/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> = TuiFrame<'a, TermionBackend<Writer>>;

/// Terminal must be used to interact with the terminal in tui applications
pub type Terminal = TuiTerminal<TermionBackend<Writer>>;
