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

/*
fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    println!("Enter a start folder: ");

    let path = env::current_dir()?;
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");
    // start_folder.insert_str(0, &file);
    // fs::canonicalize(&start_folder);

    println!("Folder: {}", start_folder);

    let mut file = File::open(start_folder).expect("File not found");

    let mut data : String = String::new();

    file.read_to_string(&mut data).expect("Something went wrong reading the file");

    println!("Contents: {}", data);

    Ok(())
} 
*/

fn main() {
    // let mut file : File = File::create("hello.txt").expect("Error encountered while creating file!");
    // file.write_all(b"what in the son of a yak").expect("Error while writing to file");
    let mut thing : String = String::from("bruh.txt");
    // (&thing as &str).what_is_this();
    let file1 = File::open(&thing as &str).expect("File not found");
    let file2 = File::open("bruh.txt").expect("File not found");

}
