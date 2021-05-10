use std::io::{stdout, Write};

use crossterm::{
    cursor::{self},
    event::{read, KeyCode, KeyEvent},
    execute, terminal, ExecutableCommand,
};
use crossterm::{
    event::{Event, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ErrorKind,
};

use crate::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        stdout.execute(cursor::MoveTo(0, 0))?;

        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            stdout.execute(cursor::MoveTo(0, 0))?;
        }

        stdout.flush().map_err(ErrorKind::IoError)
    }

    fn process_keypress(&mut self) -> Result<(), ErrorKind> {
        let pressed_key = Self::read_key()?;

        if let KeyCode::Char(e) = pressed_key.code {
            if e == 'q' && pressed_key.modifiers == KeyModifiers::CONTROL {
                self.should_quit = true;
            }
        }

        Ok(())
    }

    fn read_key() -> Result<KeyEvent, ErrorKind> {
        loop {
            let event = read()?;
            if let Event::Key(e) = event {
                return Ok(e);
            }
        }
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}

fn die(e: &ErrorKind) {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    disable_raw_mode().unwrap();

    eprintln!("error: {:?}", e);
    std::process::exit(1);
}
