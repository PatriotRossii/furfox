use crossterm::event::read;
use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
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
            match read() {
                Ok(event) => if let Event::Key(e) = event {
                    if let KeyCode::Char(c) = e.code {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }

                        if c == 'q' && e.modifiers == KeyModifiers::CONTROL {
                            break;
                        }
                    } else {
                        println!("{:?}\r", e.code);
                    }
                }
                Err(ref e) => die(e),
            }
        }

        if let Err(ref e) = disable_raw_mode() {
            die(e);
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
    std::process::exit(1);
}
