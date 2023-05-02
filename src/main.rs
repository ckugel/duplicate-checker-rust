mod file_data_img;
mod file_data_mov;
use file_data_img::FileDataImg;

extern crate image;
extern crate opencv;

use std::collections::HashMap;
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

// when true we output the files to be removed instead of removing them
const DEBUG: bool = false;

fn search_all_files(start : &str) -> () {
    let mut png_set : HashSet<Vec<u8>>  = HashSet::new();
    let mut jpeg_set: HashSet<FileDataImg> = HashSet::new();
    let mut mov_set: HashSet<FileDataMov> = HashSet::new();

    let mut png_map : HashMap<Vec<u8>, String> = HashMap::new();
    let mut jpeg_map: HashMap<FileDataImg, String> = HashMap::new();
    let mut mov_map: HashMap<FileDataMov, String> = HashMap::new();

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
        
                if DEBUG {
                    let result: Option<&String> = png_map.get(&vec);
                    if result.is_some() {
                        println!("{:?} is the same as {:?}", &path.as_os_str(), result.unwrap());
                    }
                    else {
                        png_map.insert(vec, path.to_str().unwrap().to_string());
                    }
                }
                else {
                    if png_set.contains(&vec) {
                        fs::remove_file(&path).ok();
                    }
                    else {
                        png_set.insert(vec);
                    }
                }
            },
            "jpg" | "jpeg" | "JPG" => {
                let file_data: FileDataImg = FileDataImg::new(&path.to_str().unwrap());

                if DEBUG {
                    let result: Option<&String> = jpeg_map.get(&file_data);
                    if result.is_some() {
                        println!("{:?} is the same as {:?}", &path.as_os_str(), result.unwrap());
                    }
                    else {
                        jpeg_map.insert(file_data, path.to_str().unwrap().to_string());
                    }
                }
                else {
                    if jpeg_set.contains(&file_data) {
                        fs::remove_file(&path).ok();
                    }
                    else {
                        jpeg_set.insert(file_data);
                    }
                }
            },
            "MOV" => {
                let file: File = File::open(&path.to_str().unwrap()).unwrap();
                let cap: VideoCapture = VideoCapture::from_file(&path.to_str().unwrap(), videoio::CAP_FFMPEG).unwrap();
                let mov_data: FileDataMov = FileDataMov::new(file.metadata().unwrap().size(), cap);

                if DEBUG {
                    let result: Option<&String> = mov_map.get(&mov_data);
                    if result.is_some() {
                        println!("{:?} is the same as {:?}", &path.as_os_str(), result.unwrap());
                    }
                    else {
                        mov_map.insert(mov_data, path.to_str().unwrap().to_string());
                    }
                }
                else {
                    if mov_set.contains(&mov_data) {
                        fs::remove_file(&path).ok();
                    }
                    else {
                        mov_set.insert(mov_data);
                    }
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

    search_all_files(&start_folder as &str);

    Ok(())
}

