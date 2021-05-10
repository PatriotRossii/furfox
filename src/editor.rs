use std::io::{Write, stdout};

use crossterm::{ExecutableCommand, event::{KeyEvent, read}, terminal};
use crossterm::{
    event::{Event, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ErrorKind,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        if let Err(ref e) = enable_raw_mode() {
            die(e);
        }

        loop {
            if let Err(ref error) = self.refresh_screen() {
                die(error);
            }
            if let Err(ref error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), ErrorKind> {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.flush().map_err(ErrorKind::IoError)?;
        Ok(())
    }

    fn process_keypress(&mut self) -> Result<(), ErrorKind> {
        let pressed_key = Self::read_key()?;

        Ok(match pressed_key.modifiers {
            KeyModifiers::CONTROL => self.should_quit = true,
            _ => {}
        })
    }

    fn read_key() -> Result<KeyEvent, ErrorKind> {
        loop {
            let event = read()?;
            if let Event::Key(e) = event {
                return Ok(e);
            }
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self { should_quit: false }
    }
}

fn die(e: &ErrorKind) {
    eprintln!("error: {:?}", e);
    disable_raw_mode().unwrap();
    std::process::exit(1);
}
