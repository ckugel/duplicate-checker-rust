use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use opencv::prelude::*;
use opencv::core::Mat;

use opencv::videoio::VideoCapture;

#[derive(Eq)]
pub(crate) struct FileDataMov {
    size: u64,
    frame_hashes_size: usize,
    frame_hashes: [u64; 5],
}

impl FileDataMov {
    pub fn new(size: u64, cap: VideoCapture) -> Self {
        let mut thing = FileDataMov {size, frame_hashes_size: 0, frame_hashes: [0; 5]};
        thing.compute_frames(cap);
        return thing;
    }

    pub fn compute_frame(&mut self, mat: Mat) -> u64{
        let mut hasher: DefaultHasher = DefaultHasher::new();
        let data_bytes: &[u8] = &mat.data_bytes().unwrap()[0..96];
        let mut index = 0;
        while index < data_bytes.len() {
            hasher.write_u128(u128::from_be_bytes(data_bytes[index..index+16].try_into().unwrap()));
            index += 16;
        }

        return hasher.finish();
    }

    pub fn compute_frames(&mut self, mut cap: VideoCapture) {
        while &self.frame_hashes_size < &self.frame_hashes.len() && *&cap.is_opened().unwrap() {
            let mut frame: Mat = Mat::default();
            cap.read(&mut frame).expect("Can't read frame");
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
        for i in 0..self.frame_hashes_size {
            visitor[(self.frame_hashes[i] == other.frame_hashes[i]) as usize];
        }
        return visitor[0];
    }
}
