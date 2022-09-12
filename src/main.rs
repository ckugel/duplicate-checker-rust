mod file_data_img;
mod file_data_mov;

use std::collections::hash_map::DefaultHasher;
use file_data_img::FileDataImg;

extern crate image;
extern crate opencv;

use std::collections::hash_set::HashSet;

use sha2::Sha512;
use sha2::Digest;

use std::io::stdin;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::hash::{Hash, Hasher};

use glob::glob;
use std::path::PathBuf;

use std::io::BufReader;

use opencv::videoio;

use opencv::videoio::VideoCapture;
use crate::file_data_mov::FileDataMov;

use std::os::unix::fs::MetadataExt;

fn remove_file(path: &PathBuf, mut output: &File) -> () {
    // std::fs::remove_file(&path).ok();
    output.write_all(b"Removed: ").unwrap();
    output.write_all((&path.to_str().unwrap()).as_ref()).unwrap();
    output.write_all(b"\n").unwrap();
}

fn view_png(start : &str, mut output: &File) -> () {
    let mut png_set : HashSet<Vec<u8>>  = HashSet::new();
    let mut jpeg_set: HashSet<FileDataImg> = HashSet::new();
    let mut mov_set: HashSet<FileDataMov> = HashSet::new();

    for entry in glob(&(format!("{}/**/*", start)) as &str).expect("Failed to read glob pattern") {
        let path: PathBuf = entry.unwrap();
        let extension = path.extension().unwrap_or_default().to_str().unwrap();

        match extension {
            "png" => {
                let file: File = File::open(&path).expect("Failed to open file");
        
                let mut reader: BufReader<File>  = BufReader::new(file);
                let mut buffer: Vec<u8> = Vec::new();
        
                reader.read_to_end(&mut buffer).ok();
        
                let mut hasher : Sha512 = Sha512::new();
                hasher.update(&buffer);
                let vec : Vec<u8> = hasher.finalize().to_vec();
        
                if png_set.contains(&vec) {
                    remove_file(&path, &mut output);
                }
                else {
                    png_set.insert(vec);
                }
            },
            "jpg" | "jpeg" | "JPG" => {
                let file_data: FileDataImg = FileDataImg::new(&path.to_str().unwrap());

                if jpeg_set.contains(&file_data) {
                    remove_file(&path, &mut output);
                }
                else {
                    jpeg_set.insert(file_data);
                }
            },
            "MOV" => {
                let file: File = File::open(&path.to_str().unwrap()).unwrap();
                let cap: VideoCapture = VideoCapture::from_file(&path.to_str().unwrap(), videoio::CAP_ANY).unwrap();
                let mov_data: FileDataMov = FileDataMov::new(file.metadata().unwrap().size(), cap);

                if path.to_str().unwrap().contains("IMG_E4368999999999999") {
                    let mut hasher: DefaultHasher = DefaultHasher::new();
                    mov_data.hash(&mut hasher);
                    output.write_all(b"copy: ").unwrap();
                    output.write_all((&hasher.finish().to_string()).as_ref()).unwrap();
                    output.write_all(b"\n").unwrap();
                }

                else if path.to_str().unwrap().contains("IMG_E4368") {
                    let mut hasher: DefaultHasher = DefaultHasher::new();
                    mov_data.hash(&mut hasher);
                    output.write_all(b"normal: ").unwrap();
                    output.write_all((&hasher.finish().to_string()).as_ref()).unwrap();
                    output.write_all(b"\n").unwrap();
                }

                if mov_set.contains(&mov_data) {
                    remove_file(&path, &mut output);
                }
                else {
                    mov_set.insert(mov_data);
                }
            },
            _ => continue,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    println!("Enter a start folder: ");
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");

    start_folder.truncate(start_folder.len() - 1);

    fs::canonicalize(&start_folder).ok();

    fs::remove_file("output.txt").unwrap_or_default();
    let mut out_file: File = File::create("output.txt").unwrap();

    view_png(&start_folder as &str, &mut out_file);

    Ok(())
}

