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
    pub fn from_path(path: &PathBuf) -> Result<Self, dgitError> {
        let mut file = fs::File::open(path)?;
        let mut str = String::from("");
        file.read_to_string(&mut str)?;
        let mut hasher = Sha1::new();

        // write input message
        hasher.input_str(&str);

        // read hash digest
        let hashed = hasher.result_str();

        Ok(Blob {
            hash: hashed,
            data: str.as_bytes().to_vec(),
        })
    }

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
        let index = Index {
            tree: BTreeMap::new(),
        };
        if !dest.exists() {
            return Ok(index);
        } else {
            index.read_index_file()?;
            return Ok(index);
        }
    }
    //write data to index file depend on tree

    //add data to index tree
    pub fn write_index_tree(&mut self, path: &PathBuf, hash: &str) -> Result<(), dgitError> {
        self.tree
            .insert(hash.to_string(), path.to_str().unwrap().to_string());
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
            // insert hash as key, path as value
            self.tree.insert(vec[1].to_string(), vec[0].to_string());
        }

        Ok(())
    }
}
