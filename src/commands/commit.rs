use crate::{
    lib::error::dgitError,
    types::{Commit, TreeDir},
};

pub fn commit(message: String) -> Result<(), dgitError> {
    //step 1, parse from index file to TreeDir
    //step 2, write tree to objects file
    //step 3, write commit to objects file
    //step 4, clear index file
    let mut root_tree = TreeDir::new_root()?;
    root_tree.read_index_file()?;
    root_tree.write_files()?;
    Commit::new(message, root_tree.hash).write_commit()?;
    TreeDir::clear_index_file()?;

    Ok(())
}
