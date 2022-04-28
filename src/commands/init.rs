use std::fs;
use std::io::Write;
use std::path::Path;

use crate::lib::error::dgitError;

pub fn init() -> Result<(), dgitError> {
    let dir = Path::new(".dgit");
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("branches"))?;
    fs::create_dir(dir.join("objects"))?;
    let mut head = fs::File::create(dir.join("./HEAD"))?;
    head.write_all(b"branches/master")?;
    Ok(())
}
