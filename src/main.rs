extern crate termion;
// output
use termion::raw::IntoRawMode;
use termion::{cursor, clear};
// input
use termion::event::Key;
use termion::input::TermRead;

use std::env;
use std::io;
use std::io::prelude::*;

mod frame;
mod render;

/* Things that are working: Raw mode, dispalying text
 * Things that need to work: Cursor movement, changing text
 * IDK when this will have work done on it
 */

#[allow(dead_code, unused_variables)]
fn main() {
    let filename = env::args().nth(1).unwrap();
    let frame    = frame::file_to_frame(&filename).unwrap();

    // termainal to raw mode (no output processing, draw to anywhere in terminal)
    let     stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let stdin = io::stdin();

    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

    render::draw(&mut stdout, frame).unwrap();

    for k in stdin.keys() {
        let k = k.unwrap();
        println!("{:?}\r", k);
        match k {
            Key::Char('q') |
            Key::Ctrl('c') => {break},
            _ => {},

        }
    }

    stdout.flush().unwrap();
}
