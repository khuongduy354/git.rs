use std::path::PathBuf;

use crate::{lib::error::dgitError, types::Blob, types::Index};
pub fn add(path: &PathBuf) -> Result<(), dgitError> {
    let blob = Blob::from_path(&path)?;
    let mut index = Index::new();
    index.write_index_tree(path, &blob.hash)?;
    blob.write_blob()?;
    Ok(())
}
