use crate::duplicate_package::DuplicatePackage;
use core::num;
use std::sync::Mutex;


pub(crate) struct DeletionManager {
    duplicates: Mutex<Vec<DuplicatePackage>>
}

impl DeletionManager {
    pub fn new() -> DeletionManager {
        return DeletionManager {
            duplicates: Mutex::new(Vec::new())
        }
    }

    pub fn add_duplicate(&mut self, duplicate: DuplicatePackage) {
        /*
        let number_of_times_failed: u8 = 0;
        let succeeded: bool = false;
        while (!succeeded || number_of_times_failed < 5) {
            match self.duplicates.lock() {
                Ok(mut vector) => {
                    vector.push(duplicate);
                    succeeded = true;
                },
                Err(e) => {
                    number_of_times_failed += 1;
                }
            }
        }
        if !succeeded {
            panic!("Failed to add a duplicate package to deletion manager");
        }
        */
        self.duplicates.lock().unwrap().push(duplicate);
    }

    pub fn is_empty(&self) -> bool {
        return self.duplicates.lock().unwrap().len() == 0;
    }

    pub fn pop_most_recent(&mut self) -> DuplicatePackage {
        /*
        match self.duplicates.lock() {
            Ok(vector) => return vector.pop().unwrap(),
            Err(e) => {
                println!("You have a threading error somewhere :|");
                panic!(e);
            }
        }
        */
        return self.duplicates.lock().unwrap().pop().unwrap();
    }
}