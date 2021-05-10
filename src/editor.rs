use crossterm::event::KeyCode;
use crossterm::{event::KeyModifiers, terminal::disable_raw_mode, ErrorKind};

use crate::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
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
        Terminal::clear_screen()?;
        Terminal::cursor_position(0, 0)?;

        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0)?;
        }

        Terminal::flush().map_err(ErrorKind::IoError)
    }

    fn process_keypress(&mut self) -> Result<(), ErrorKind> {
        let pressed_key = Terminal::read_key()?;

        if let KeyCode::Char(e) = pressed_key.code {
            if e == 'q' && pressed_key.modifiers == KeyModifiers::CONTROL {
                self.should_quit = true;
            }
        }

        Ok(())
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
    Terminal::clear_screen().unwrap();
    disable_raw_mode().unwrap();

    eprintln!("error: {:?}", e);
    std::process::exit(1);
}
