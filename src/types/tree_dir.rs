use std::{
    borrow::BorrowMut,
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Component, PathBuf},
};

use crypto::{digest::Digest, sha1::Sha1};

use crate::lib::error::dgitError;

use super::Blob;

pub struct TreeDir {
    pub full_path: PathBuf,
    pub hash: String,
    pub trees: BTreeMap<String, TreeDir>, //other trees file/dir name and its values
    pub blobs: BTreeMap<String, String>,  //blobs name and hash
}

impl TreeDir {
    pub fn new(full_path: PathBuf) -> Self {
        TreeDir {
            full_path,
            hash: String::from(""),
            blobs: BTreeMap::new(),
            trees: BTreeMap::new(),
        }
    }
    pub fn new_root() -> Self {
        TreeDir {
            full_path: PathBuf::from("."),
            hash: String::from(""),
            blobs: BTreeMap::new(),
            trees: BTreeMap::new(),
        }
    }

    //return the furthest dir available
    pub fn find_dir(&mut self, path: &mut PathBuf) -> Result<(&mut TreeDir, PathBuf), dgitError> {
        let mut current_dir = self;
        let mut new_path = PathBuf::from(".");
        //iterate left to right from path
        for component in path.components().borrow_mut() {
            if let Component::Normal(name) = component {
                let name = name.to_str().expect("find_dir: name");

                //move to next dir if found
                if current_dir.trees.contains_key(name) {
                    current_dir = current_dir
                        .trees
                        .get_mut(name)
                        .expect("find_dir: current_dir");
                    new_path = new_path.join(name);
                }
                //return current dir, if not
                else {
                    //if path is not changed, return original path
                    new_path = if new_path == PathBuf::from("") {
                        path.to_path_buf()
                    } else {
                        //e.g: src/types/data is path
                        //     src/types is new_path
                        //     return /data
                        path.to_path_buf()
                            .strip_prefix(new_path)
                            .expect("find_dir: new_path")
                            .to_path_buf()
                    };
                    let tuple = (current_dir, new_path);
                    return Ok(tuple);
                }
            }
        }
        Err(dgitError::NoDirectory)
    }

    //insert 1 dir/file to the tree
    pub fn insert_item(&mut self, rel_path: &PathBuf) -> Result<(), dgitError> {
        //step 1 make sure file/dir exists
        //step 2 insert depend on file/dir
        let name = rel_path.to_str().expect("read_index_file: name");
        let full_path = self.full_path.join(rel_path);
        if full_path.is_dir() {
            self.trees.insert(name.to_string(), TreeDir::new(full_path));
        } else if full_path.is_file() {
            self.blobs.insert(
                name.to_string(),
                Blob::from_path(&full_path)
                    .expect("read_index_file: Blob")
                    .hash,
            );
        }
        Ok(())
    }
    //get content as string for current dir
    pub fn get_content(&self) -> Result<String, dgitError> {
        let mut content = String::new();
        //prepare dirs content, and recursively  write_files for every dir inside
        for (name, tree) in &self.trees {
            let line = format!("tree {} {}\n", tree.hash, name);
            content = content + &line;
        }
        //prepare blobs content
        for (name, hash) in &self.blobs {
            let line = format!("blob {} {}\n", hash, name);
            content = content + &line;
        }
        //remove the last \n
        content.pop();
        content.pop();

        Ok(content)
    }

    //hash all data in the tree
    pub fn update_hash(&mut self) -> Result<(), dgitError> {
        //if current dir has dir inside -> recursively hash it
        if self.trees.len() > 0 {
            for (_, tree) in &mut self.trees {
                tree.update_hash()?;
            }
        }
        //update root tree hash
        let content = self.get_content()?;
        let mut hasher = Sha1::new();
        hasher.input_str(&content);
        self.hash = hasher.result_str();
        Ok(())
    }
    pub fn clear_index_file() -> Result<(), dgitError> {
        let dest = PathBuf::from(".dgit").join("index");
        if dest.exists() {
            fs::File::create(dest)?;
        }
        Ok(())
    }

    //write trees/blobs of entire tree to files
    pub fn write_files(&mut self) -> Result<(), dgitError> {
        //get current dir content
        let content = self.get_content()?;

        //hash content
        let mut hasher = Sha1::new();
        hasher.input_str(&content);
        let hashed = hasher.result_str();

        //write content to file with hash as name
        let dir_path = PathBuf::from(".dgit").join("objects").join(&hashed[0..2]);
        let file_path = dir_path.join(&hashed[2..]);
        if !dir_path.exists() {
            fs::create_dir(&dir_path)?
        };
        fs::File::create(file_path)?.write_all(content.as_bytes())?;

        //recursively do this for every dir inside
        for (_, tree) in &mut self.trees {
            tree.write_files()?;
        }
        Ok(())
    }

    //read index file, and update its to dir tree
    pub fn read_index_file(&mut self) -> Result<(), dgitError> {
        //read index file content
        let dest = PathBuf::from(".dgit").join("index");
        if !dest.exists() {
            fs::File::create(&dest)?;
        }
        let content = fs::read_to_string(dest)?;

        //iterate each line
        for line in content.lines() {
            //take hash (left) and path (right), as a vec with length of 2
            let vec = line.split(" ").collect::<Vec<&str>>();
            if vec.len() != 2 {
                return Err(dgitError::InvalidIndex);
            }

            //check if some of the dir in the path is existed
            //e.g: if src/types/ is existed, and we need to add src/types/index.rs
            //it'll return src/types path and correspond tree
            //in case not, it'll received full path
            let mut full_path = PathBuf::from(vec[1]);
            let parent = self.find_dir(&mut full_path)?;
            //destructure, dir and its path
            let (mut parent, path) = parent;
            for component in path.components() {
                if let Component::Normal(x) = component {
                    let path = PathBuf::from(x);
                    //add item
                    parent.insert_item(&path)?;

                    //move to next dir
                    if path.is_dir() {
                        let path = path.to_str().expect("read_index_file: path");
                        parent = parent.trees.get_mut(path).expect("read_index_file: parent");
                    };
                }
            }
        }
        //hash data in the tree
        self.update_hash()?;
        Ok(())
    }
}
