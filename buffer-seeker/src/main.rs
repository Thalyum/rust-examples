use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;

fn buffer_seeker<W: Read + Seek>(reader: &mut W, offset: u64, size: u64) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    reader.seek(SeekFrom::Start(offset)).unwrap();
    reader.take(size).read_to_end(&mut buffer).unwrap();

    buffer
}

fn main() {
    let v: Vec<u8> = vec![0, 1, 2, 0xff, 0, 0, 20, 40];

    let mut reader = Cursor::new(v);

    let e = buffer_seeker(&mut reader, 0, 4);
    println!("{:?}", e);

    let e = buffer_seeker(&mut reader, 2, 3);
    println!("{:?}", e);
}
