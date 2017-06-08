use std::io;
use std::io::prelude::*;
use termion::cursor;
use frame;

#[allow(dead_code)]
pub fn draw<W: Write>(interface: &mut W, frame: frame::Frame) -> io::Result<()> {
    for (y, row) in frame.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
           let xpos = (x + 1) as u16;
           let ypos = (y + 1) as u16;
           write!(
               interface,
               "{}{}",
               cursor::Goto(xpos, ypos),
               c
           )?;
        }
    }
    Ok(())
}
