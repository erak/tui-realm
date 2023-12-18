//! ## terminal
//!
//! Cross platform Terminal helper

use std::fmt::Arguments;
use std::io::{Error, Write};

use thiserror::Error;

use crate::Terminal;

// -- types
pub type TerminalResult<T> = Result<T, TerminalError>;

#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("cannot connect to output")]
    CannotConnect(Error),
    #[error("cannot initialize termion")]
    Termion(Error),
    #[error("cannot enter alternate mode")]
    CannotEnterAlternateMode,
    #[error("cannot leave alternate mode")]
    CannotLeaveAlternateMode,
    #[error("cannot toggle raw mode")]
    CannotToggleRawMode,
    #[error("cannot clear screen")]
    CannotClear,
    #[error("backend doesn't support this command")]
    Unsupported,
}

/// An helper around `Terminal` to quickly setup and perform on terminal.
/// You can opt whether to use or not this structure to interact with the terminal
/// Anyway this structure is 100% cross-backend compatible and is really easy to use, so I suggest you to use it.
/// If you need more advance terminal command, you can get a reference to it using the `raw()` and `raw_mut()` methods.
pub struct TerminalBridge {
    terminal: Terminal,
}

impl TerminalBridge {
    /// Instantiates a new Terminal bridge
    pub fn new(writer: Writer) -> TerminalResult<Self> {
        Ok(Self {
            terminal: Self::adapt_new_terminal(writer)?,
        })
    }

    /// Instantiates a default Terminal bridge
    pub fn default() -> TerminalResult<Self> {
        Ok(Self {
            terminal: Self::adapt_default_terminal()?,
        })
    }

    /// Enter in alternate screen using the terminal adapter
    pub fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        self.adapt_enter_alternate_screen()
    }

    /// Leave the alternate screen using the terminal adapter
    pub fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        self.adapt_leave_alternate_screen()
    }

    /// Clear the screen
    pub fn clear_screen(&mut self) -> TerminalResult<()> {
        self.adapt_clear_screen()
    }

    /// Enable terminal raw mode
    pub fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        self.adapt_enable_raw_mode()
    }

    /// Disable terminal raw mode
    pub fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        self.adapt_disable_raw_mode()
    }

    /// Returna an immutable reference to the raw `Terminal` structure
    pub fn raw(&self) -> &Terminal {
        &self.terminal
    }

    /// Return a mutable reference to the raw `Terminal` structure
    pub fn raw_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}

pub struct Writer {
    target: Box<dyn Write>,
}

impl Writer {
    pub fn new(target: Box<dyn Write>) -> Self {
        Self { target }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.target.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.target.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.target.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        self.target.write_fmt(fmt)
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}
