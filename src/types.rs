use crypto::{self, digest::Digest, sha1::Sha1};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::lib::error::dgitError;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

impl Blob {
    //make blob from path
    pub fn from_path(path: &PathBuf) -> Result<Self, dgitError> {
        let mut file = fs::File::open(path)?;

        let mut str = String::from("");
        file.read_to_string(&mut str)?;
        let mut hasher = Sha1::new();

        // write input message
        // let input = str.to_owned() + path.to_str().unwrap();
        hasher.input_str(&str);

        // read hash digest
        let hashed = hasher.result_str();

        Ok(Blob {
            hash: hashed,
            data: str.as_bytes().to_vec(),
        })
    }
    //write blob to file
    pub fn write_blob(&self) -> Result<(), dgitError> {
        let blob_dir = Path::new(".dgit").join("objects").join(&self.hash[0..2]);

        if !blob_dir.exists() {
            fs::create_dir(&blob_dir)?;
        };
        let blob_file_path = &blob_dir.join(&self.hash[2..]);
        let mut blob_file = File::create(blob_file_path)?;
        blob_file.write_all(&self.data)?;
        Ok(())
    }
}
pub struct Index {
    tree: BTreeMap<String, String>,
}
impl Index {
    pub fn new() -> Result<Self, dgitError> {
        let dest = Path::new(".dgit").join("index");
        let mut index = Index {
            tree: BTreeMap::new(),
        };
        if !dest.exists() {
            File::create(dest)?;
            return Ok(index);
        } else {
            index.read_index_file()?;
            return Ok(index);
        }
    }

    //update index tree
    pub fn write_index(&mut self, path: &PathBuf) -> Result<(), dgitError> {
        let metadata = fs::metadata(path).expect("write_index: metadata");

        if metadata.is_file() {
            let blob = Blob::from_path(path).expect("write_index: blob");
            self.update_file_tree(path.to_owned(), &blob.hash)?;
            blob.write_blob()?;
            return Ok(());
        } else if metadata.is_dir() {
            self.update_dir_tree(path.to_owned())?;
            return Ok(());
        }
        Err(dgitError::NoDirectory)
    }

    //update 1 file to tree
    pub fn update_file_tree(&mut self, path: PathBuf, hash: &str) -> Result<(), dgitError> {
        self.tree
            .insert(path.to_str().unwrap().to_string(), hash.to_string()); // path as key, hash as value
        Ok(())
    }

    //update all files inside a dir to tree
    pub fn update_dir_tree(&mut self, path: PathBuf) -> Result<(), dgitError> {
        let inside_dir = fs::read_dir(path).expect("Here");
        for entry in inside_dir {
            let entry = entry?;
            if entry.path().is_dir() {
                self.update_dir_tree(entry.path())?;
            } else {
                let blob = Blob::from_path(&entry.path())?;
                blob.write_blob()?;
                self.update_file_tree(entry.path(), &blob.hash)?;
            }
        }
        Ok(())
    }
    //write to file
    pub fn write_index_file(&self) -> Result<(), dgitError> {
        let dest = Path::new(".dgit").join("index");
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(dest)?;
        for (hash, path) in self.tree.iter() {
            file.write_all(path.as_bytes())?;
            file.write_all(b" ")?;
            file.write_all(hash.as_bytes())?;
            file.write_all(b"\n")?;
        }

        Ok(())
    }

    //read index from the file and add it to the tree
    pub fn read_index_file(&mut self) -> Result<(), dgitError> {
        let dest = Path::new(".dgit").join("index");
        let content = fs::read_to_string(dest)?;
        for line in content.lines() {
            let vec = line.split(" ").collect::<Vec<&str>>();
            if vec.len() != 2 {
                return Err(dgitError::InvalidIndex);
            }
            //check if file is deleted, if not add to index tree
            let path = PathBuf::from(vec[1]);
            if path.exists() {
                self.tree.insert(vec[1].to_string(), vec[0].to_string());
            }
        }

        Ok(())
    }

    pub fn clear_index_file() -> Result<(), dgitError> {
        let dest = Path::new(".dgit").join("index");
        if dest.exists() {}
        Ok(())
    }
}
pub struct TreeDir {
    hash: String,
    trees: Vec<TreeDir>,
    blobs: Vec<Blob>,
}
