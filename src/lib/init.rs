use super::error::rgitError;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn init() -> Result<(), rgitError> {
    let dir = Path::new(".dgit");
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("HEADS"))?;
    let mut head = fs::File::create(dir.join("./HEADS/head"))?;
    head.write_all(b"HEADS/master")?;
    Ok(())
}
