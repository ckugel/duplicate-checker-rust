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

type DeletionManager = Arc<Mutex<Vec<DuplicatePackage>>>;

// fragile folders is a feature where if a duplicate file is found it will delete the one in the fragile folder as opposed to deleting the one it saw first
const USE_FRAGILE_FOLDERS: bool = true;

fn deletion_queue_manager(to_delete: DeletionManager, manager_thread: JoinHandle<()>) -> () {
    loop {
        let mut to_delete = to_delete.lock().unwrap();
        if to_delete.len() > 0 {
            let package: DuplicatePackage = to_delete.pop().unwrap();
            println!("Would you like to delete (1) '{:?}' or (2) '{:?}'", package.get_file_one(), package.get_file_two());

            let mut input_was_valid: bool = false;
            let mut file_num: u8 = 1;
    
            while !input_was_valid {
                print!("\nHow many folders would you like to declare fragile? (0 for none)\n");
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
                println!("deleting {:?}", package.get_file_two());
                // fs::remove_file(package.get_file_two()).ok();
            }
            else {
                println!("deleting {:?}", package.get_file_one());
                // fs::remove_file(package.get_file_one()).ok();
            }
        }
        // queue is empty and the manager thread is finished, we can exit
        else if manager_thread.is_finished() {
            break;
        }
        else {
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}

fn deletion_manager_loop(manager: DeletionManager, fragile_folders: Vec<String>, main_thread: JoinHandle<()>, to_delete: DeletionManager) -> () {
    loop {
        let mut manager = manager.lock().unwrap();
        if manager.len() > 0 {
            let package: DuplicatePackage = manager.pop().unwrap();
            if USE_FRAGILE_FOLDERS {
                let mut was_removed: bool = false;
                for folder in &fragile_folders {
                    if package.get_file_one().contains(folder) {
                        println!("deleting {:?}", package.get_file_one());
                        // fs::remove_file(package.get_file_one()).ok();
                        was_removed = true;
                    }
                    if package.get_file_two().contains(folder) {
                        println!("deleting {:?}", package.get_file_two());
                        // fs::remove_file(package.get_file_two()).ok();
                        was_removed = true;
                    }
                }
                // pair to be removed did not get removed during the fragile folder check, so we add it to the deletion queue and prompt the user
                if !was_removed {
                    to_delete.lock().unwrap().push(package);
                }
            }
            else {
                println!("deleting {:?}", package.get_file_one());
                fs::remove_file(package.get_file_one()).ok();
            }
        }
        else {
            if main_thread.is_finished() {
                break;
            }
            thread::sleep(time::Duration::from_millis(1));
        }

    }
}

fn search_all_files(start : &str, manager: DeletionManager) -> () {
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
                        manager.lock().unwrap().push(DuplicatePackage::new(path.to_str().unwrap().to_string(), result.to_string()));
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
                        manager.lock().unwrap().push(DuplicatePackage::new(path.to_str().unwrap().to_string(), result.to_string()));
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
                        manager.lock().unwrap().push(DuplicatePackage::new(path.to_str().unwrap().to_string(), result.to_string()));
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
            /*println!("pass in a fragile folder: ");
            let mut fragile_folder: String = String::new();
            stdin().read_line(&mut fragile_folder).expect("Failed to read line");
            fragile_folder.truncate(fragile_folder.len() - 1);
            */
            let fragile_folder: String = String::from("/var/mnt/bigssd/testing/fragile-boy");
            fs::canonicalize(&fragile_folder).ok();
            folders.push(fragile_folder);
        }
        fragile_folders = folders;
    }
    else {
        fragile_folders = Vec::new();
    }

    let manager: DeletionManager = Arc::new(Mutex::new(Vec::new()));
    let manager_copy: DeletionManager = manager.clone();

    let deletion_queue: DeletionManager = Arc::new(Mutex::new(Vec::new()));
    let deletion_queue_copy: DeletionManager = deletion_queue.clone();

    println!("\nEnter a start folder: ");
    
    /* stdin().read_line(&mut start_folder).expect("Failed to read line");

    start_folder.truncate(start_folder.len() - 1);
    */

    start_folder = String::from("/var/mnt/bigssd/testing");
    fs::canonicalize(&start_folder).ok();

    let handle = thread::spawn(
        move || {
            search_all_files(&start_folder as &str, manager);
        }
    );

    let manager_handle: JoinHandle<()> = thread::spawn(
        move || deletion_manager_loop(manager_copy, fragile_folders, handle, deletion_queue_copy)
    );

    deletion_queue_manager(deletion_queue, manager_handle);

    Ok(())
}

