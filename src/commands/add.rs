use std::path::PathBuf;

use crate::{lib::error::dgitError, types::Index};
pub fn add(path: &PathBuf) -> Result<(), dgitError> {
    let mut index = Index::new()?;
    // update TreeDir struct, and write blobs to disk at the same time
    index.write_index(path)?;

    // write index file to disk
    index.write_index_file()?;
    Ok(())
}
