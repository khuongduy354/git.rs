use std::{fs, path::PathBuf, str::FromStr};

use crate::lib::{error::dgitError, file_service};

fn print_commit_content(hash: &str, content: &str) {
    println!("Commit: {}", hash);
    println!(
        "Message: {}",
        content.lines().nth(1).expect("Invalid commit file")
    );
}
pub fn log() -> Result<(), dgitError> {
    // read HEAD -> get branch
    let head_path = PathBuf::from_str(".dgit/HEAD").expect("Failed to get HEAD file path");
    let head_content = fs::read_to_string(head_path)?;

    // read branches/branch
    let branch = &head_content.lines().next().expect("Failed to get branch")[9..];

    // get commit -> print, check 4 line and rerun
    let branch_path = PathBuf::from_str(&format!(".dgit/branches/{}", branch))
        .expect("Failed to get branch file path");
    let mut commit_hash = fs::read_to_string(branch_path)?;

    while commit_hash != "" {
        let commit_path = file_service::get_blob_path_from_hash(&commit_hash);
        let commit_content = fs::read_to_string(commit_path).expect("Cannot read commit file");
        let prev_commit = commit_content.lines().nth(3);

        print!("\n\n");
        print_commit_content(&commit_hash, &commit_content);
        print!("------------------------------------\n");

        if let Some(prev_commit) = prev_commit {
            commit_hash = prev_commit.to_string();
        } else {
            commit_hash = "".to_string();
        }
    }

    Ok(())
}
