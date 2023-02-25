use std::{
    fs::File,
    io::{stdin, BufReader, Read},
    path::Path,
};

const PATH: &str = "test.txt";

fn main() {
    let path = Path::new(PATH);
    let file = File::open(path).unwrap();
    let mut bufreader = BufReader::new(file);

    let mut content: Vec<u8> = Vec::new();
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        bufreader.read_to_end(&mut content).unwrap();
        println!("file content:\n{}", String::from_utf8_lossy(&content));
    }
}
