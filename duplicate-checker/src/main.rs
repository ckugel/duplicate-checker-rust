extern crate image;
extern crate img_hash;

use img_hash::image::EncodableLayout;
use img_hash::{HasherConfig, HashAlg};

use std::collections::hash_map::DefaultHasher;
use std::collections::hash_set::HashSet;
use std::hash::{Hash, Hasher};

use sha2::Sha512;
use sha2::Digest;

use std::io::stdin;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

use std::{fs, hash};
use std::env;

use glob::glob;
use std::path::PathBuf;

use sha2::Sha256;
use std::io;

use std::io::BufReader;

use string_builder::Builder;

use base64ct::{Base64, Encoding};


fn viewPNG(start : &str) -> () {
    let mut set : HashSet<Vec<u8>>  = HashSet::new();
    for entry in glob(&(format!("{}/**/*.png", start)) as &str).expect("Failed to read glob pattern") {

        // let mut hasher = Sha256::new();
        // let mut file = fs::File::open(&entry.unwrap().as_path()).unwrap();

        // let bytes_written = io::copy(&mut file, &mut hasher);
        // let hash_bytes = hasher.finalize();

        let path = &entry.unwrap();

        let file: File = File::open(path).expect("Failed to read line");
        let mut reader: BufReader<File>  = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_to_end(&mut buffer);

        let mut hasher : Sha512 = Sha512::new();
        hasher.update(&buffer);
        let vec : Vec<u8> = hasher.finalize().to_vec();

        if set.contains(&vec) {
            std::fs::remove_file(path).ok();
        }
        else {
            set.insert(vec);
        }

        

        // let mut builder = Builder::new(64);

        // let hashedValue = Sha512::digest(&entry.unwrap().into_os_string().into_string().unwrap());

        // for item in hashedValue {
        //     builder.append(char::from(item));
        // }

        // println!("Vector: {:?} \n", hashedValue);
        // println!("{} \n\n\n", builder.string().unwrap());

        // if (map.contains_key(&hashedValue)) {
            
        // }

        // println!("{:?}", entry.unwrap().display());
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


    // let vec : Vec<u8> = vec![65, 34, 127, 99, 2, 34];
    // let vec2 : Vec<u8> = vec![65, 34, 127, 99, 2, 34];

    // let mut hash = DefaultHasher::new();
    // let mut hash2 = DefaultHasher::new();

    // vec.hash(&mut hash);
    // vec2.hash(&mut hash2);

    // println!("hash 1: {:?}", hash.finish());
    // println!("hash 2: {:?}", hash2.finish());
    

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
