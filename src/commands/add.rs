use std::path::PathBuf;

use crate::{lib::error::dgitError, types::Index};
pub fn add(path: &PathBuf) -> Result<(), dgitError> {
    //step 1, update tree based on path, and write blob at the same time
    //step 2, write index file based on tree
    let mut index = Index::new()?;
    index.write_index(path)?; //update index tree
    index.write_index_file()?;
    Ok(())
}
