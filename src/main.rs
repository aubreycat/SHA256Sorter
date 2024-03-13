use std::env;
use std::fs;
use std::io;
use std::path::Path;
use sha2::{Sha256, Digest};

fn calculate_checksum(file_path: &str) -> Result<String, io::Error> {
    let contents = fs::read(file_path)?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Ok(format!("{:x}", hasher.finalize()))
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: program.exe <folder>");
        return Ok(());
    }

    let dir_path = &args[1];
    let mut files: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if let Ok(checksum) = calculate_checksum(&file_path.to_string_lossy()) {
                        files.push((file_name_str.to_string(), checksum));
                    }
                }
            }
        }
    }

    files.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Files sorted by SHA256 checksum:");
    println!("---------------------------------");
    
    for (file_name, checksum) in &files {
        println!("{:<30} {}", file_name, checksum);
    }

    Ok(())
}