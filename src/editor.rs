use crossterm::event::{KeyEvent, read};
use crossterm::{
    event::{Event, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    ErrorKind,
};

pub struct Editor {}

impl Editor {
    pub fn run() {
        if let Err(ref e) = enable_raw_mode() {
            die(e);
        }

        loop {
            if let Err(ref error) = Self::process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress() -> Result<(), ErrorKind> {
        let pressed_key = Self::read_key()?;

        Ok(match pressed_key.modifiers {
            KeyModifiers::CONTROL => panic!("Program end"),
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
        Self {}
    }
}

fn die(e: &ErrorKind) {
    eprintln!("error: {:?}", e);
    disable_raw_mode().unwrap();
    std::process::exit(1);
}
