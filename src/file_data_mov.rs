use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use opencv::prelude::*;
use opencv::core::Mat;

use opencv::videoio::VideoCapture;

pub(crate) struct FileDataMov {
    size: u64,
    frame_hashes_size: usize,
    frame_hashes: [u64; 5],
    cap: Box<VideoCapture>,
    // hashed: u64 = 0,
}

impl FileDataMov {
    pub fn new(size: u64, cap: Box<VideoCapture>) -> Self {
        let mut thing = FileDataMov {size, frame_hashes_size: 0, frame_hashes: [0; 5], cap};
        thing.compute_frames();
        return thing;
    }

    pub fn compute_frame(&mut self, mat: Mat) -> u64{
        let mut hasher: DefaultHasher = DefaultHasher::new();
        let data_bytes: &[u8] = &mat.data_bytes().unwrap()[0..80];
        let mut index = 0;
        while index < data_bytes.len() {
            hasher.write_u128(u128::from_be_bytes(data_bytes[index..index+16].try_into().unwrap()));
            index += 16;
        }

        return hasher.finish();
    }

    pub fn compute_frames(&mut self) {
        while &self.frame_hashes_size < &self.frame_hashes.len() && *&self.cap.is_opened().unwrap() {
            let mut frame: Mat = Mat::default();
            self.cap.read(&mut frame).expect("Can't read frame");
            self.frame_hashes[self.frame_hashes_size % self.frame_hashes.len()] = self.compute_frame(frame);
            self.frame_hashes_size += 1;
        }
    }
}

impl Hash for FileDataMov {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..self.frame_hashes_size {
            state.write_u64(self.frame_hashes[i]);
        }
    }
}

impl PartialEq<Self> for FileDataMov {
    fn eq(&self, other: &Self) -> bool {
        let mut visitor: [bool; 2] = [true, true];
        visitor[(self.size == other.size) as usize] = false;
        if self.frame_hashes_size != other.frame_hashes_size {
            return false;
        }
        unsafe {
            let obj1: *const u64 = self.frame_hashes.as_ptr();
            let obj2: *const u64 = self.frame_hashes.as_ptr();
            for i in 0..self.frame_hashes_size {
                visitor[(obj1.offset(i as isize) != obj2.offset(i as isize)) as usize] = false;
            }
        }
        return visitor[0];
    }
}

impl Eq for FileDataMov {}
