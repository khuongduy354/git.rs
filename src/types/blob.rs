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
