extern crate termion;

use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

pub struct Display {
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Display {
    pub fn new() -> Display {
        Display {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn draw_byte(&mut self, byte: u8, x: u16, y: u16) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(x, y),
            from_byte(byte)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
    }
}

// 01010000 -> " * *"
fn from_byte(mut byte: u8) -> String {
    let one = if byte >= 128 { "*" } else { " " };
    byte <<= 1;
    let two = if byte >= 128 { "*" } else { " " };
    byte <<= 1;
    let three = if byte >= 128 { "*" } else { " " };
    byte <<= 1;
    let four = if byte >= 128 { "*" } else { " " };
    format!("{}{}{}{}", one, two, three, four)
}
