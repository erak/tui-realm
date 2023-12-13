//! ## Terminal
//!
//! terminal bridge adapter for termion

use crate::terminal::Writer;
use std::io::stdout;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

use crate::terminal::{TerminalBridge, TerminalError, TerminalResult};
use crate::tui::backend::TermionBackend;
use crate::Terminal;

impl TerminalBridge {
    pub(crate) fn adapt_default_terminal() -> TerminalResult<Terminal> {
        let stream = stdout()
            .into_raw_mode()
            .map_err(|err| TerminalError::CannotConnect(err))?
            .into_alternate_screen()
            .map_err(|err| TerminalError::CannotConnect(err))?;
        let stream = MouseTerminal::from(stream);
        let writer = Writer::new(Box::new(stream));

        Terminal::new(TermionBackend::new(writer)).map_err(|err| TerminalError::CannotConnect(err))
    }

    pub(crate) fn adapt_new_terminal(writer: Writer) -> TerminalResult<Terminal> {
        let stream = writer
            .into_raw_mode()
            .map_err(|err| TerminalError::CannotConnect(err))?
            .into_alternate_screen()
            .map_err(|err| TerminalError::CannotConnect(err))?;
        let writer = Writer::new(Box::new(stream));

        Terminal::new(TermionBackend::new(writer)).map_err(|err| TerminalError::CannotConnect(err))
    }

    pub(crate) fn adapt_enter_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_leave_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_clear_screen(&mut self) -> TerminalResult<()> {
        self.raw_mut()
            .clear()
            .map_err(|_| TerminalError::CannotClear)
    }

    pub(crate) fn adapt_enable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_disable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }
}
