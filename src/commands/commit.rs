use std::{hash::Hash, path::PathBuf};

use crate::{lib::error::dgitError, types::TreeDir};

pub fn commit(message: String) -> Result<(), dgitError> {
    //step 1, parse from index file to TreeDir
    //step 2, write commit to files
    let mut root_tree = TreeDir::new("root".to_string(), PathBuf::from("."))?;
    root_tree.read_index_file()?;
    println!(
        "{} ",
        root_tree
            .trees
            .get("testf")
            .unwrap()
            .blobs
            .get("test2.txt")
            .unwrap()
    );
    Ok(())
}
