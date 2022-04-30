use std::{fs, path::PathBuf};

fn main() {
    let dest = PathBuf::from(".dgit").join("index");
    fs::File::create(dest).unwrap();
}
