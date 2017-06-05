extern crate termion;
use termion::raw::IntoRawMode;
use termion::cursor;

use std::env;
use std::io;
use std::io::prelude::*;

mod frame;

#[allow(dead_code, unused_variables)]
fn main() {
    let filename = env::args().nth(1).unwrap();
    let frame    = frame::file_to_frame(&filename).unwrap();

    // termainal to raw mode (no output processing, draw to anywhere in terminal)
    let     stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();

    for (y, row) in frame.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let xpos = (x + 1) as u16; // need a typecast
            let ypos = (y + 1) as u16;
            write!(stdout, "{}{}", cursor::Goto(xpos, ypos), c).unwrap();
        }
    }
    write!(stdout, "{}", cursor::Show).unwrap();


    stdout.flush().unwrap();
}
