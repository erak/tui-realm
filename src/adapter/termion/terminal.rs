//! ## Terminal
//!
//! terminal bridge adapter for termion

use std::io::stdout;

use ratatui::layout::Rect;
// use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
// use termion::screen::IntoAlternateScreen;
use crate::tui::{TerminalOptions, Viewport};

use crate::terminal::{TerminalBridge, TerminalError, TerminalResult};
use crate::tui::backend::TermionBackend;
use crate::Terminal;

impl TerminalBridge {
    pub(crate) fn adapt_new_terminal() -> TerminalResult<Terminal> {
        let stdout = stdout()
            .into_raw_mode()
            .map_err(|err| TerminalError::CannotConnectStdout(err.to_string()))?;
        // .into_alternate_screen()
        // .map_err(|_| TerminalError::CannotConnectStdout)?;
        // let stdout = MouseTerminal::from(stdout);
        // Terminal::new(TermionBackend::new(stdout)).map_err(|err| TerminalError::CannotConnectStdout(err.to_string()))
        Terminal::with_options(
            TermionBackend::new(stdout),
            TerminalOptions {
                viewport: Viewport::Fixed(Rect::new(0, 0, 80, 10)),
            },
        )
        .map_err(|err| TerminalError::CannotConnectStdout(err.to_string()))
    }

    pub(crate) fn adapt_enter_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_leave_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_clear_screen(&mut self) -> TerminalResult<()> {
        println!("clear......");
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
