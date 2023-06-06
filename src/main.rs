mod file_data_img;
mod file_data_mov;
mod deletion_manager;
mod duplicate_package;

use crate::file_data_mov::FileDataMov;
use crate::file_data_img::FileDataImg;
use crate::deletion_manager::DeletionManager;
use crate::duplicate_package::DuplicatePackage;

extern crate image;
extern crate opencv;

use std::collections::HashMap;
use std::collections::hash_set::HashSet;

use std::{thread, time};

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

use std::os::unix::fs::MetadataExt;

// when true we output the files to be removed instead of removing them
const DEBUG: bool = true;
// fragile folders is a feature where if a duplicate file is found it will delete the one in the fragile folder as opposed to deleting the one it saw first
const USE_FRAGILE_FOLDERS: bool = true;

fn deletion_manager_loop(mut manager: Box<DeletionManager>, fragile_folders: Vec<String>) -> () {
    loop {
        if !manager.is_empty() {
            let package: DuplicatePackage = manager.pop_most_recent();
            if DEBUG {
                println!("{:?} is the same as {:?}", package.get_file_one(), package.get_file_two());
            }
            else {
                if USE_FRAGILE_FOLDERS {

                }
                else {

                }
            }

        }
        else {
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}

fn search_all_files(start : &str, manager: Box<DeletionManager>) -> () {
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
                    match jpeg_map.get(&file_data) {
                        Some(result) => {
                            println!("{:?} is the same as {:?}", &path.as_os_str(), result)
                        }
                        _ => {
                            // we ignore the results of insert because we already check if the key is present in the map. 
                            //TODO: Look into if this code could be replaced with a match on the insert call
                            jpeg_map.insert(file_data, path.to_str().unwrap().to_string());
                        }
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
                    match mov_map.get(&mov_data) {
                        Some(result) => {
                            println!("{:?} is the same as {:?}", &path.as_os_str(), result)
                        }
                        _ => {
                            // we ignore the results of insert because we already check if the key is present in the map. 
                            //TODO: Look into if this code could be replaced with a match on the insert call
                            mov_map.insert(mov_data, path.to_str().unwrap().to_string());
                        }
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

    let fragile_folders: Vec<String>;

    // rust's compiler not generating any of this in asm if based on the constant is amazing
    if USE_FRAGILE_FOLDERS {
        // get the number of fragile folders that the user will pass in
        let mut input_was_valid: bool = false;
        let mut num_fragile_folders: u16 = 0;

            while !input_was_valid {
            print!("\nHow many folders would you like to declare fragile? (0 for none)\n");
            let mut num_buf: String = String::new();
            stdin().read_line(&mut num_buf).expect("Failed to read line");
            match num_buf.parse::<u16>() {
                Ok(value) => {
                    input_was_valid = true;
                    num_fragile_folders = value;
                }
                Err(_) => {
                    print!("\nnot a valid input, input needs to be a number\n");
                }
            }
        }

        // get the fragile folders from the user
        let mut folders: Vec<String> = Vec::new();
        for _ in 0..num_fragile_folders {
            println!("pass in a fragile folder: ");
            let mut fragile_folder: String = String::new();
            stdin().read_line(&mut fragile_folder).expect("Failed to read line");
            fragile_folder.truncate(fragile_folder.len() - 1);
            fs::canonicalize(&fragile_folder).ok();
            folders.push(fragile_folder);
        }
        fragile_folders = folders;
    }
    else {
        fragile_folders = Vec::new();
    }

    print!("\nEnter a start folder: ");
    
    stdin().read_line(&mut start_folder).expect("Failed to read line");

    start_folder.truncate(start_folder.len() - 1);

    fs::canonicalize(&start_folder).ok();

    let manager: Box<DeletionManager> = Box::new(DeletionManager::new());

    // in thread start
    // deletion_manager_loop(manager, fragile_folders);

    search_all_files(&start_folder as &str, manager);

    Ok(())
}

