mod file_data_img;
mod file_data_mov;

use crate::file_data_mov::FileDataMov;
use crate::file_data_img::FileDataImg;

extern crate image;
extern crate opencv;

use std::collections::HashMap;

use std::sync::Mutex;

use exif::Error;
use sha2::Sha512;
use sha2::Digest;
use sha2::digest::typenum::Length;

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

fn search_all_files(start : &str, png_map: &mut HashMap<Vec<u8>, Mutex<Vec<String>>>, jpeg_map: &mut HashMap<FileDataImg, Mutex<Vec<String>>>, mov_map: &mut HashMap<FileDataMov, Mutex<Vec<String>>>) -> () {
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
                       result.lock().unwrap().push(path.to_str().unwrap().to_string());
                    }
                    _ => {
                        let to_store: Mutex<Vec<String>> = Mutex::new(Vec::new());
                        to_store.lock().unwrap().push(path.to_str().unwrap().to_string());
                        png_map.insert(vec, to_store);
                    }
                }
            },
            "jpg" | "jpeg" | "JPG" => {
                let file_data: FileDataImg = FileDataImg::new(&path.to_str().unwrap());
                match jpeg_map.get(&file_data) {
                    Some(result) => {
                        result.lock().unwrap().push(path.to_str().unwrap().to_string());
                    }
                    _ => {
                        let to_store: Mutex<Vec<String>> = Mutex::new(Vec::new());
                        to_store.lock().unwrap().push(path.to_str().unwrap().to_string());
                        jpeg_map.insert(file_data, to_store);
                    }
                }
            },
            "MOV" => {
                let file: File = File::open(&path.to_str().unwrap()).unwrap();
                let cap: VideoCapture = VideoCapture::from_file(&path.to_str().unwrap(), videoio::CAP_FFMPEG).unwrap();
                let mov_data: FileDataMov = FileDataMov::new(file.metadata().unwrap().size(), cap);

                match mov_map.get(&mov_data) {
                    Some(result) => {
                        result.lock().unwrap().push(path.to_str().unwrap().to_string());
                    }
                    _ => {
                        let to_store: Mutex<Vec<String>> = Mutex::new(Vec::new());
                        to_store.lock().unwrap().push(path.to_str().unwrap().to_string());
                        mov_map.insert(mov_data, to_store);
                    }
                }
            },
            _ => continue,
        }
    }
}

fn prompt_user_destroy(to_delete_from: Mutex<Vec<String>>) {
    let to_delete_from: Vec<String> = to_delete_from.lock().unwrap().to_vec();
    println!("Select the one to keep by passing in it's number:");
    for i in 0..to_delete_from.len() {
        println!("({}): {}", i, file);
    }

    let mut input_was_valid: bool = false;
    let mut num_to_keep: u16 = 0;
    while !input_was_valid {
        println!("Please enter a number in the range of the files above:");
        let mut num_buf: String = String::new();
        let attempt = stdin().read_line(&mut num_buf);
        match attempt {
            Error(_) => "Could not read line",
            OK(value) => {
                num_buf.truncate(num_buf.len() - 1);
                match num_buf.parse::<u16>() {
                    Ok(value) => {
                        if value >= to_delete_from.len() {
                            print!("\nnot a valid input, input needs to be a number in the range of the files above\n");
                            continue;
                        }
                        num_to_keep = value;
                        input_was_valid = true;
                    }
                    Err(_) => {
                        print!("\nnot a valid input, input needs to be a number\n");
                    }
                };
            }
        }
        for i in 0..to_delete_from.len() {
            if i != num_to_keep {
                fs::remove_file(to_delete_from[i]).ok();
            }
        }
    }

}

fn fragile_folder_check(fragile_folders: Vec<String>, png_map: HashMap<Vec<u8>, Mutex<Vec<String>>>, jpeg_map: HashMap<FileDataImg, Mutex<Vec<String>>>, mov_map: HashMap<FileDataMov, Mutex<Vec<String>>>) {
    // every single png
    for png in png_map.into_values() {
        let png: Vec<String> = png.lock().unwrap().to_vec();
        if png.len() < 2 {
            continue;
        } else {
            let mut contains_non_fragile_folder: bool = false;
            for file in png {
                    if !fragile_folders.contains(&file) {
                        contains_non_fragile_folder = true;
                    }
            }

            if contains_non_fragile_folder {
                for file in png {
                    fs::remove_file(file).ok();
                }
            } else {
                for file in png {
                    fs::remove_file(file).ok();
                }
            }
        }
    }

    // every single jpeg

    // every single mov
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
    } else {
        fragile_folders = Vec::new();
    }

    println!("\nEnter a start folder: ");
    stdin().read_line(&mut start_folder).expect("Failed to read line");
    start_folder.truncate(start_folder.len() - 1);
    fs::canonicalize(&start_folder).ok();

    let mut png_map: HashMap<Vec<u8>, Mutex<Vec<String>>> = HashMap::new();
    let mut jpeg_map: HashMap<FileDataImg, Mutex<Vec<String>>> = HashMap::new();
    let mut mov_map: HashMap<FileDataMov, Mutex<Vec<String>>> = HashMap::new();

    search_all_files(&start_folder, &mut png_map, &mut jpeg_map, &mut mov_map);
    fragile_folder_check(fragile_folders, png_map, jpeg_map, mov_map);

    Ok(())
}

