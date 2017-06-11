use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub type Row   = Vec<char>;
pub type Frame = Vec<Row>;

pub fn file_to_frame(filename: &str) -> io::Result<Frame> {
    let     file = File::open(filename)?;
    let mut file = BufReader::new(file);

    let mut frame = Vec::new();
    let mut buff  = String::new();

    while file.read_line(&mut text)? > 0 { // read all bytes, append to string
        let mut row = Vec::new();          // have to allocate here since we're transferring ownership
        for c in text.chars() {
            row.push(c);
        }
        frame.push(row);
        text.clear(); // remember to clear the buffer after each! (doesn't deallocate)
    }
    Ok(frame)
}
