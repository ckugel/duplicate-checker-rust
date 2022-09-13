mod file_data_img;
mod file_data_mov;
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

use glob::glob;
use std::path::PathBuf;

use std::io::BufReader;

use opencv::videoio;

use opencv::videoio::VideoCapture;
use crate::file_data_mov::FileDataMov;

use std::os::unix::fs::MetadataExt;


fn view_png(start : &str) -> () {
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
                    fs::remove_file(&path).ok();
                }
                else {
                    png_set.insert(vec);
                }
            },
            "jpg" | "jpeg" | "JPG" => {
                let file_data: FileDataImg = FileDataImg::new(&path.to_str().unwrap());

                if jpeg_set.contains(&file_data) {
                    fs::remove_file(&path).ok();
                }
                else {
                    jpeg_set.insert(file_data);
                }
            },
            "MOV" => {
                let file: File = File::open(&path.to_str().unwrap()).unwrap();
                let cap: VideoCapture = VideoCapture::from_file(&path.to_str().unwrap(), videoio::CAP_ANY).unwrap();
                let mov_data: FileDataMov = FileDataMov::new(file.metadata().unwrap().size(), cap);

                if mov_set.contains(&mov_data) {
                    fs::remove_file(&path).ok();
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

    view_png(&start_folder as &str);

    Ok(())
}

