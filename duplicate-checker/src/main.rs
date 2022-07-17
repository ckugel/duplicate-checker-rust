extern crate image;
extern crate img_hash;

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
use img_hash::HasherConfig;

fn view_png(start : &str) -> () {
    let mut set : HashSet<Vec<u8>>  = HashSet::new();
    for entry in glob(&(format!("{}/**/*.png", start)) as &str).expect("Failed to read glob pattern") {
        let path: PathBuf = entry.unwrap();
        let file: File = File::open(&path).expect("Failed to read line");

        let mut reader: BufReader<File>  = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_to_end(&mut buffer).ok();

        let mut hasher : Sha512 = Sha512::new();
        hasher.update(&buffer);
        let vec : Vec<u8> = hasher.finalize().to_vec();

        if set.contains(&vec) {
            print!("Removing {}... ", path.to_str().unwrap());
            std::fs::remove_file(path).ok();
            print!("Done \n");
        } else {
            set.insert(vec);
        }
    }
}

fn view_jpeg(start: &str, extension: &str) {
    let mut set: HashSet<Box<String>> = HashSet::new();
    let hasher = HasherConfig::new().to_hasher();

    for entry in glob(&format!("{}/**/*.{}", start, extension) as &str).expect("Failed to read glob pattern") {
        let path: PathBuf = entry.unwrap();
        let name: String =String::from(path.as_path().to_str().unwrap());

        let image = image::open(&path).unwrap();
        let hash: Box<String> = Box::new(hasher.hash_image(&image).to_base64());

        if set.contains(&hash) {
            print!("Removing {}... ", name);
            std::fs::remove_file(path).ok();
            print!("Done \n");
        }
        else {
            set.insert(hash);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    println!("Enter a start folder: ");
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");

    start_folder.truncate(start_folder.len() - 1);

    fs::canonicalize(&start_folder).ok();

    // let path1: String = String::from("/mnt/3468843A6883F8BE/pictures-videos/vacations-holidays/San Francisco 2022/IMG_3226.JPG");
    // let path2: String = String::from("/mnt/3468843A6883F8BE/pictures-videos/vacations-holidays/Cali 2022/IMG_3226.JPG");

    // let path1: String = String::from("folder1/Screenshot from 2021-09-02 22-25-30.png");
    // let path2: String = String::from("folder1/Screenshot from 2021-10-17 23-22-40.png");

    // let image1 = image::open(path1).unwrap();
    // let image2 = image::open(path2).unwrap();

    // let hasher = HasherConfig::new().to_hasher();

    // let hash1 = hasher.hash_image(&image1);
    // let hash2 = hasher.hash_image(&image2);

    // println!("Image1 hash: {}", hash1.to_base64());
    // println!("Image2 hash: {}", hash2.to_base64());

    // println!("Hamming Distance: {}", hash1.dist(&hash2));   

    view_png(&start_folder as &str);
    view_jpeg(&start_folder as &str, "JPG");
    view_jpeg(&start_folder as &str, "jpeg");
    view_jpeg(&start_folder as &str, "jpg");
    

    Ok(())
} 
