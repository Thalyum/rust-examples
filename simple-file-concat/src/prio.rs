use bincode::serialize;
use ring::digest::{Context, Digest, SHA256};
use serde::{Deserialize, Serialize};
use std::fs::{metadata, read, File};
use std::io::{BufReader, Read, Write};
use std::mem::{size_of, size_of_val};
use std::path::Path;
use verbosity::Verbosity;

#[derive(Debug, Serialize)]
struct FileHeader {
    filename: String,
    checksum: [u8; 32],
    filesize: usize,
}

const MAGIC: [u8; 4] = [0x50, 0x52, 0x49, 0x4F]; //PRIO

#[derive(Debug, Serialize)]
struct ArchiveBlob {
    magic: [u8; 4],
    ser_header_list: Vec<u8>,
    payload: Vec<u8>,
}

// https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
fn sha256_digest(path: &String) -> Digest {
    let input = File::open(path).unwrap();
    let mut reader = BufReader::new(input);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}

fn init_file_header(input: &String, filesize: usize) -> FileHeader {
    let mut checksum = [0; 32];
    sha256_digest(input)
        .as_ref()
        .iter()
        .zip(checksum.iter_mut())
        .for_each(|(b, ptr)| *ptr = *b);

    let filename = Path::new(input)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    FileHeader {
        filename,
        checksum,
        filesize,
    }
}

pub fn main_concat(inputs: Vec<&String>, output_path: &String) {
    if Path::new(output_path).exists() {
        panic!("Error: Output file '{}' exists already", output_path);
    }

    let mut payload: Vec<u8> = Vec::new();
    let mut header_list: Vec<FileHeader> = Vec::new();

    for input in inputs {
        if !Path::new(input).exists() {
            panic!("Error: '{}' does not exist", input);
        }

        println!("Adding '{}' to archive", input);

        let mut buffer: Vec<u8> = read(input).unwrap();

        let file_header = init_file_header(input, buffer.len());

        if Verbosity::level() == Verbosity::Verbose {
            println!(
                "---\nfilename: {}\tfile size: {}\nsha256: {:02x?}\n---",
                file_header.filename, file_header.filesize, file_header.checksum
            );
        };

        payload.append(&mut buffer);
        header_list.push(file_header);
    }

    // serialize the header list
    let ser_header_list = serialize(&header_list).unwrap();

    let mut new_archive = ArchiveBlob {
        magic: MAGIC,
        ser_header_list,
        payload,
    };

    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut new_archive.magic.to_vec());
    buffer.append(&mut new_archive.ser_header_list);
    buffer.append(&mut new_archive.payload);

    let mut output = File::create(output_path).unwrap();
    output.write(&buffer).unwrap();
}

pub fn main_extract() {
    println!("extract !");
}
