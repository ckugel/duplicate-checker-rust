extern crate image;
extern crate img_hash;

use img_hash::{ImageHash, HashType};

use std::io::stdin;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

use std::fs;
use std::env;

fn writeAndHash(start : String) {
    if let Ok(entries) = fs::read_dir(start) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(file_type) = entry.file_type() {
                    // Now let's show our entry's file type!
                    println!("{:?}: {:?}", entry.path(), file_type);
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    println!("Enter a start folder: ");

    let path = env::current_dir()?;

    let file : String = format!("{}/", path.display());
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");
    // start_folder.insert_str(0, &file);
    // fs::canonicalize(&start_folder);

    println!("Folder: {}", start_folder);

    let contents : String = fs::read_to_string(start_folder).expect("Something went wrong reading the file");

    println!("Contents: {}", contents);

    Ok(())
}