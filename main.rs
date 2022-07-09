extern crate image;
extern crate img_hash;

use img_hash::{HasherConfig, HashAlg};

use std::io::stdin;
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

fn writeAndHash(start : String) {
    let mut seen : Vec<File> = Vec::new();

}

fn main() -> std::io::Result<()> {
    let mut start_folder : String = String::new();

    // println!("Enter a start folder: ");
    
    // stdin().read_line(&mut start_folder).ok();

    // let mut file = File::open(start_folder)?;

    let mut file : File = File::open("/mnt/c/Users/caleb/Documents/bruh.txt")?;

    let mut contents : String = String::new();
    file.read_to_string(&mut contents)?;

    println!("Contents: {}", contents);

    Ok(())
}