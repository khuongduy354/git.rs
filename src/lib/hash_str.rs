use crypto::{digest::Digest, sha1::Sha1};

pub fn hash_str(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input);
    hasher.result_str()
}
