//! ## termion
//!
//! this module contains the adapters for termion

extern crate termion;

mod event;
mod listener;
mod terminal;

// -- export
use std::io::Stdout;

pub use listener::TermionInputListener;
// use termion::input::MouseTerminal;
use termion::raw::RawTerminal;
// use termion::screen::AlternateScreen;

// use ratatui::prelude::*;

use super::{Event, Key, KeyEvent, KeyModifiers};
use crate::tui::backend::TermionBackend;
use crate::tui::{Frame as TuiFrame, Terminal as TuiTerminal};

// -- Frame

/// Frame represents the Frame where the view will be displayed in
// pub type Frame<'a> =
//     TuiFrame<'a, TermionBackend<MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>>>;

// pub type Frame<'a> =
//     TuiFrame<'a, TermionBackend<MouseTerminal<RawTerminal<Stdout>>>>;

pub type Frame<'a> = TuiFrame<'a>;

/// Terminal must be used to interact with the terminal in tui applications
// pub type Terminal =
//     TuiTerminal<TermionBackend<MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>>>;

pub type Terminal = TuiTerminal<TermionBackend<RawTerminal<Stdout>>>;
