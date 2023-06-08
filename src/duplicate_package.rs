pub(crate) struct DuplicatePackage {
    file_one: String,
    file_two: String,
}

impl DuplicatePackage {
    pub fn new(file_one: String, file_two: String) -> DuplicatePackage {
        return DuplicatePackage {
            file_one: file_one,
            file_two: file_two,
        }
    }

    pub fn get_file_one(&self) -> &String {
        return &self.file_one;
    }

    pub fn get_file_two(&self) -> &String {
        return &self.file_two;
    }
}

