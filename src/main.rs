mod file_data_img;
mod file_data_mov;
mod duplicate_package;

use crate::file_data_mov::FileDataMov;
use crate::file_data_img::FileDataImg;
use crate::duplicate_package::DuplicatePackage;

extern crate image;
extern crate opencv;

use std::collections::HashMap;
use std::thread::JoinHandle;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use image::codecs::jpeg;
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

// fragile folders is a feature where if a duplicate file is found it will delete the one in the fragile folder as opposed to deleting the one it saw first
const USE_FRAGILE_FOLDERS: bool = true;
const DEBUG: bool = true;

fn search_all_files(start : &str, fragile_folders: Vec<String>) -> () {
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
                match png_map.get(&vec) {
                    Some(result) => {
                        thread::spawn(move || {
                            let file_name: String = path.to_str().unwrap().to_string();
                            // result is the path of the file in the map
                            // file_name is the path of the file we are currently looking at
                            for folder in &fragile_folders {
                                if result.contains(folder) {
                                    println!("deleting {:?}", result);
                                    if !DEBUG {
                                        fs::remove_file(result).ok();
                                    }
                                    png_map.insert(vec, file_name); // clear out the now removed file to avoid phantom caching
                                    break;
                                }
                                else if file_name.contains(folder) {
                                    println!("deleting {:?}", &file_name);
                                    if !DEBUG {
                                        fs::remove_file(&file_name).ok();
                                    }
                                    break;
                                }
                                else {
                                    let mut input_was_valid: bool = false;
                                    let mut file_num: u8 = 1;
                            
                                    while !input_was_valid {
                                        println!("Would you like to delete (1) '{:?}' or (2) '{:?}'", &result, &file_name);
                                        let mut num_buf: String = String::new();
                                        stdin().read_line(&mut num_buf).expect("Failed to read line");
                                        num_buf.truncate(num_buf.len() - 1);
                                        match num_buf.parse::<u8>() {
                                            Ok(value) => {
                                                input_was_valid = true;
                                                file_num = value;
                                            }
                                            Err(_) => {
                                                print!("\nnot a valid input, input needs to be a number\n");
                                            }
                                        }
                                    }
                                    if file_num >= 2 {
                                        println!("deleting {:?}", &file_name);
                                        if !DEBUG {
                                            fs::remove_file(&file_name).ok();
                                        }
                                    }
                                    else {
                                        println!("deleting {:?}", &result);
                                        if !DEBUG {
                                            fs::remove_file(&result).ok();
                                        }
                                        png_map.insert(vec, file_name); // clear out the now removed file to avoid phantom caching

                                    }
                                }
                            }
                        });
                    }
                    _ => {
                        png_map.insert(vec, path.to_str().unwrap().to_string());
                    }
                }
            },
            "jpg" | "jpeg" | "JPG" => {
                let file_data: FileDataImg = FileDataImg::new(&path.to_str().unwrap());
                match jpeg_map.get(&file_data) {
                    Some(result) => {
                        thread::spawn(move || {
                            let file_name: String = path.to_str().unwrap().to_string();
                            // result is the path of the file in the map
                            // file_name is the path of the file we are currently looking at
                            for folder in &fragile_folders {
                                if result.contains(folder) {
                                    println!("deleting {:?}", result);
                                    if !DEBUG {
                                        fs::remove_file(result).ok();
                                    }
                                    jpeg_map.insert(file_data.clone(), file_name.clone()); // clear out the now removed file to avoid phantom caching
                                    break;
                                }
                                else if file_name.contains(folder) {
                                    println!("deleting {:?}", &file_name);
                                    if !DEBUG {
                                        fs::remove_file(&file_name).ok();
                                    }
                                    break;
                                }
                                else {
                                    let mut input_was_valid: bool = false;
                                    let mut file_num: u8 = 1;
                            
                                    while !input_was_valid {
                                        println!("Would you like to delete (1) '{:?}' or (2) '{:?}'", &result, &file_name);
                                        let mut num_buf: String = String::new();
                                        stdin().read_line(&mut num_buf).expect("Failed to read line");
                                        num_buf.truncate(num_buf.len() - 1);
                                        match num_buf.parse::<u8>() {
                                            Ok(value) => {
                                                input_was_valid = true;
                                                file_num = value;
                                            }
                                            Err(_) => {
                                                print!("\nnot a valid input, input needs to be a number\n");
                                            }
                                        }
                                    }
                                    if file_num >= 2 {
                                        println!("deleting {:?}", &file_name);
                                        if !DEBUG {
                                            fs::remove_file(&file_name).ok();
                                        }
                                    }
                                    else {
                                        println!("deleting {:?}", &result);
                                        if !DEBUG {
                                            fs::remove_file(&result).ok();
                                        }
                                        jpeg_map.insert(file_data, file_name); // clear out the now removed file to avoid phantom caching
                                        //TODO: OPTIMIZATION FOR LATER: fs::remove_file(jpeg_map.insert(file_data, file_name).unwrap()).ok();

                                    }
                                }
                            }
                        });
                    }
                    _ => {
                        // we ignore the results of insert because we already check if the key is present in the map. 
                        //TODO: Look into if this code could be replaced with a match on the insert call
                        jpeg_map.insert(file_data, path.to_str().unwrap().to_string());
                    }
                }
            },
            "MOV" => {
                let file: File = File::open(&path.to_str().unwrap()).unwrap();
                let cap: VideoCapture = VideoCapture::from_file(&path.to_str().unwrap(), videoio::CAP_FFMPEG).unwrap();
                let mov_data: FileDataMov = FileDataMov::new(file.metadata().unwrap().size(), cap);

                match mov_map.get(&mov_data) {
                    Some(result) => {
                            let file_name: String = path.to_str().unwrap().to_string();
                            // result is the path of the file in the map
                            // file_name is the path of the file we are currently looking at
                            for folder in &fragile_folders {
                                if result.contains(folder) {
                                    println!("deleting {:?}", result);
                                    if !DEBUG {
                                        fs::remove_file(result).ok();
                                    }
                                    mov_map.insert(mov_data, file_name); // clear out the now removed file to avoid phantom caching
                                    break;
                                }
                                else if file_name.contains(folder) {
                                    println!("deleting {:?}", &file_name);
                                    if !DEBUG {
                                        fs::remove_file(&file_name).ok();
                                    }
                                    break;
                                }
                                else {
                                    let mut input_was_valid: bool = false;
                                    let mut file_num: u8 = 1;
                            
                                    while !input_was_valid {
                                        println!("Would you like to delete (1) '{:?}' or (2) '{:?}'", &result, &file_name);
                                        let mut num_buf: String = String::new();
                                        stdin().read_line(&mut num_buf).expect("Failed to read line");
                                        num_buf.truncate(num_buf.len() - 1);
                                        match num_buf.parse::<u8>() {
                                            Ok(value) => {
                                                input_was_valid = true;
                                                file_num = value;
                                            }
                                            Err(_) => {
                                                print!("\nnot a valid input, input needs to be a number\n");
                                            }
                                        }
                                    }
                                    if file_num >= 2 {
                                        println!("deleting {:?}", &file_name);
                                        if !DEBUG {
                                            fs::remove_file(&file_name).ok();
                                        }
                                    }
                                    else {
                                        println!("deleting {:?}", &result);
                                        if !DEBUG {
                                            fs::remove_file(&result).ok();
                                        }
                                        mov_map.insert(mov_data, file_name); // clear out the now removed file to avoid phantom caching
                                        //TODO: OPTIMIZATION FOR LATER: fs::remove_file(mov_map.insert(mov_data, file_name).unwrap()).ok();

                                    }
                                }
                            }
                    }
                    _ => {
                        // we ignore the results of insert because we already check if the key is present in the map. 
                        //TODO: Look into if this code could be replaced with a match on the insert call
                        mov_map.insert(mov_data, path.to_str().unwrap().to_string());
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
            num_buf.truncate(num_buf.len() - 1);
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

    println!("\nEnter a start folder: ");
    stdin().read_line(&mut start_folder).expect("Failed to read line");
    start_folder.truncate(start_folder.len() - 1);
    fs::canonicalize(&start_folder).ok();

        search_all_files(&start_folder as &str, fragile_folders);

    Ok(())
}

