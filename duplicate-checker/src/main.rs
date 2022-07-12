use std::collections::hash_set::HashSet;

use sha2::Sha512;
use sha2::Digest;

use std::io::stdin;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

use std::fs;

use glob::glob;
use std::path::PathBuf;


use std::io::BufReader;

fn view_img(start : &str, extension: &str) -> () {
    let mut set : HashSet<Vec<u8>>  = HashSet::new();
    for entry in glob(&(format!("{}/**/*.{}", start, extension)) as &str).expect("Failed to read glob pattern") {

        let path: &PathBuf = &entry.unwrap();

        let file: File = File::open(path).expect("Failed to read line");
        let mut reader: BufReader<File>  = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_to_end(&mut buffer).ok();

        let mut hasher : Sha512 = Sha512::new();
        hasher.update(&buffer);
        let vec : Vec<u8> = hasher.finalize().to_vec();

        if set.contains(&vec) {
            std::fs::remove_file(path).ok();
        }
        else {
            set.insert(vec);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    println!("Enter a start folder: ");
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");

    fs::canonicalize(&start_folder).ok();

    start_folder.truncate(start_folder.len() - 1);


    view_img(&start_folder as &str, "png");
    view_img(&start_folder as &str, "JPG");
    view_img(&start_folder as &str, "jpeg");
    

    Ok(())
} 
