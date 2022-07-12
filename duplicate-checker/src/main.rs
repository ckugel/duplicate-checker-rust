extern crate image;
extern crate img_hash;

use img_hash::{HasherConfig, HashAlg};

use std::collections::hash_map::HashMap;

use std::io::stdin;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

use std::fs;
use std::env;

use glob::glob;
use std::path::PathBuf;

fn viewPNG(start : &str) -> () {
    let mut map : HashMap<img_hash::ImageHash, PathBuf> = HashMap::new();
    let hasher = HasherConfig::new().to_hasher();
    for entry in glob(&(format!("{}/**/*.png", start)) as &str).expect("Failed to read glob pattern") {

        let image1 = image::open(&entry.unwrap().as_path()).unwrap();

        let hashedValue : img_hash::ImageHash = hasher.hash_image(&image1);
        if (map.contains_key(&hashedValue)) {
            
        }
        println!("{:?}", entry.unwrap().display());
    }
}


fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    println!("Enter a start folder: ");

    let path = env::current_dir()?;
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");
    // start_folder.insert_str(0, &file);
    // fs::canonicalize(&start_folder);

    start_folder.truncate(start_folder.len() - 1);

    // println!("Folder: {}", start_folder);

    // let mut file = File::open(&start_folder as &str).expect("File not found");

    // let mut data : String = String::new();

    // file.read_to_string(&mut data).expect("Something went wrong reading the file");

    // println!("Contents: {}", data);

    viewPNG(&start_folder as &str);

    Ok(())
} 


    /*
fn main() {
    // let mut file : File = File::create("hello.txt").expect("Error encountered while creating file!");
    // file.write_all(b"what in the son of a yak").expect("Error while writing to file");

    let mut thing : String = String::new();
    
    stdin().read_line(&mut thing).expect("Failed to read line");

    thing.truncate(thing.len() - 1);

    println!("{}", thing);
    println!("bruh.txt");
    assert_eq!(thing, "bruh.txt");
*//*
    let mut file1 = File::open(&thing as &str).expect("File not found");

    let mut data : String = String::new();

    file1.read_to_string(&mut data).expect("Something went wrong reading the file");

    println!("Contents: {}", data);



}
*/
