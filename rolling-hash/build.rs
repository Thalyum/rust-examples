use std::{
    fs::File,
    io::{Read, Write},
};

// Create a random file of 10KiB
// Also save a part of it: the target
const INPUT_SZ: usize = 10 * 1024;
const TARGET_OFF: usize = 8 * 1024;
const TARGET_SZ: usize = 512;

fn main() {
    let mut rand_buf = vec![0u8; INPUT_SZ];
    let mut random = File::open("/dev/urandom").unwrap();

    random.read_exact(&mut rand_buf).unwrap();

    let target = &rand_buf[TARGET_OFF..TARGET_OFF + TARGET_SZ];

    let mut target_file = File::create("target").unwrap();
    target_file.write_all(&target).unwrap();
    let mut input_file = File::create("input").unwrap();
    input_file.write_all(&rand_buf).unwrap();
}
