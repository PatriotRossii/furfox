use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyEvent},
    execute,
    terminal::{self, enable_raw_mode},
    ErrorKind,
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, ErrorKind> {
        let size = terminal::size()?;

        enable_raw_mode()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() -> Result<(), ErrorKind> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn cursor_position(x: u16, y: u16) -> Result<(), ErrorKind> {
        execute!(stdout(), cursor::MoveTo(x, y))?;
        Ok(())
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<KeyEvent, ErrorKind> {
        loop {
            let event = read()?;
            if let Event::Key(e) = event {
                return Ok(e);
            }
        }
    }
}
