use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(".");
    let path = path.join("dir");
    let path = path.join("dir2");
    println!("{}", path.display());
    let path = path.strip_prefix("./dir/").unwrap().to_path_buf();
    println!("{}", path.to_str().unwrap());
}
