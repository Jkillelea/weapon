use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub type Row   = Vec<char>;
pub type Frame = Vec<Row>;

pub fn file_to_frame(filename: &str) -> io::Result<Frame> {
    let     file = File::open(filename)?;
    let mut file = BufReader::new(file);

    let mut frame = Frame::new();
    let mut buff  = String::new();

    while file.read_line(&mut buff)? > 0 { // read all bytes, append to string
        let mut row = Row::new();          // have to allocate here since we're transferring ownership
        for c in buff.chars() {
            row.push(c);
        }
        frame.push(row);
        buff.clear(); // remember to clear the buffer after each! (doesn't deallocate)
    }
    Ok(frame)
}
