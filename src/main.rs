use crossterm::{ErrorKind, event::{Event, KeyCode, KeyModifiers}, terminal::enable_raw_mode};
use crossterm::event::read;

fn die(e: ErrorKind) {
    eprintln!("error: {:?}", e);
    std::process::exit(1);
}

fn main() {
    enable_raw_mode().unwrap();
    
    loop {
        match read() {
            Ok(event) => {
                match event {
                    Event::Key(e) => {
                        if let KeyCode::Char(c) = e.code {
                            if c.is_control() {
                                println!("{:?}\r", c as u8);
                            } else {
                                println!("{:?} ({})\r", c as u8, c);
                            }
        
                            if c == 'q' && e.modifiers == KeyModifiers::CONTROL {
                                break
                            }
                        } else {
                            println!("{:?}\r", e.code);
                        }            
                    },
                    _ => {}
                }        
            }
            Err(e) => {
                die(e)
            }
        }
    }
}
