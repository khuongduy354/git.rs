use std::{path::PathBuf, str::FromStr};

pub fn get_blob_path_from_hash(hash: &str) -> PathBuf {
    return PathBuf::from(".dgit")
        .join("objects")
        .join(&hash[0..2])
        .join(&hash[2..]);
}
