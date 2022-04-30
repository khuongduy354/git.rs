use crate::{lib::error::dgitError, types::TreeDir};

pub fn commit(message: String) -> Result<(), dgitError> {
    //step 1, parse from index file to TreeDir
    //step 2, write commit to files
    let mut root_tree = TreeDir::new_root();
    root_tree.read_index_file()?;
    root_tree.write_files()?;

    Ok(())
}
