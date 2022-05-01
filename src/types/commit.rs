use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use crate::lib::{error::dgitError, hash_str::hash_str};

pub struct Commit {
    message: String,
    tree_hash: String,
    parent_hash: String,
}

impl Commit {
    pub fn new(message: String, tree_hash: String) -> Self {
        let parent_hash = Commit::get_latest_commit();
        Commit {
            message,
            tree_hash,
            parent_hash,
        }
    }
    pub fn get_latest_commit() -> String {
        //get commit from a branch (master as default)
        let master_file = PathBuf::from(".dgit").join("branches").join("master");
        if !master_file.exists() {
            return "".to_string();
        } else {
            let mut file = fs::File::open(master_file).expect("Failed to open master file");
            let mut hash = String::new();
            file.read_to_string(&mut hash)
                .expect("Failed to read master file");
            return hash;
        }
    }
    //write commit as hash file
    pub fn write_commit(&self) -> Result<(), dgitError> {
        //prepare content
        let mut commit_string = String::new();
        commit_string.push_str("commit");
        commit_string.push_str("\n");
        commit_string.push_str(&self.message);
        commit_string.push_str("\n");
        commit_string.push_str(&self.tree_hash);
        commit_string.push_str("\n");
        commit_string.push_str(&self.parent_hash);

        //make commit file
        let hashed_commit = hash_str(&commit_string);
        let dir_dest = PathBuf::from(".dgit")
            .join("objects")
            .join(&hashed_commit[0..2]);
        let file_dest = dir_dest.join(&hashed_commit[2..]);
        if !dir_dest.exists() {
            fs::create_dir(&dir_dest)?;
        }
        let mut commit_file = fs::File::create(file_dest)?;

        //write to file
        commit_file.write_all(commit_string.as_bytes())?;

        //update master
        let dest = PathBuf::from(".dgit").join("branches").join("master");
        if !dest.exists() {
            fs::File::create(&dest)?;
        }
        let mut master_file = fs::File::options().write(true).open(dest)?;
        master_file
            .write_all(hashed_commit.as_bytes())
            .expect("Failed to write to master file");

        Ok(())
    }
}
