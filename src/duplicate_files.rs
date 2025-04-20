use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

fn main() -> io::Result<()> {
    let directory = "./"; // Update this path
    let file_hashes = find_duplicates(directory)?;
    delete_duplicates(file_hashes)?;
    Ok(())
}

fn find_duplicates(directory: &str) -> io::Result<HashMap<String, Vec<PathBuf>>> {
    let mut file_hashes = HashMap::new();

    for entry in WalkDir::new(directory) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            let hash = hash_file(path)?;
            file_hashes
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_path_buf());
        }
    }
    Ok(file_hashes)
}

fn delete_duplicates(file_hashes: HashMap<String, Vec<PathBuf>>) -> io::Result<()> {
    for (hash, paths) in file_hashes {
        if paths.len() > 1 {
            println!("Found {} duplicates for hash {}:", paths.len() - 1, hash);
            for path in &paths[1..] {
                fs::remove_file(path)?;
                println!("Deleted duplicate: {:?}", path);
            }
        }
    }
    Ok(())
}

fn hash_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    } // Fixed: Added closing brace for loop block
    
    Ok(format!("{:x}", hasher.finalize()))
} // Fixed: Proper function closure