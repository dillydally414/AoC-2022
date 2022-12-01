use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn file_to_str(filename: &str) -> String {
    let path = Path::new(filename);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path).unwrap();

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

pub fn write_to_file(filename: &str, contents: String) -> () {
    let mut file = File::create(filename).unwrap();
    file.write(contents.as_bytes()).unwrap();
}