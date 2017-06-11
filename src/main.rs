extern crate termion;
use termion::raw::IntoRawMode;
use termion::{cursor, clear};

use std::env;
use std::io;
use std::io::prelude::*;

use std::thread::sleep;
use std::time::Duration;

mod frame;
mod render;

#[allow(dead_code, unused_variables)]
fn main() {
    let filename = env::args().nth(1).unwrap();
    let frame    = frame::file_to_frame(&filename).unwrap();

    // termainal to raw mode (no output processing, draw to anywhere in terminal)
    let     stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

    render::draw(&mut stdout, frame).unwrap();

    sleep(Duration::from_secs(5));

    stdout.flush().unwrap();
}
