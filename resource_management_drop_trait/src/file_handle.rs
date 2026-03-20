// TODO: Define a public FileHandle struct with a public filename field (String)
pub struct FileHandle {
    // TODO: Add the filename field
    pub filename: String,
}

impl FileHandle {
    // TODO: Implement the new associated function
    // It should:
    // 1. Print "Opening file: {filename}"
    // 2. Return a new FileHandle instance
    pub fn new(filename: &str) -> FileHandle {
        // TODO: Implement this function
        println!("Opening file: {}", filename);
        FileHandle {
            filename: filename.to_string(),
        }   
    }
}

// TODO: Implement the Drop trait for FileHandle
// When dropped, it should print "Closing file: {filename}"
impl Drop for FileHandle {
    fn drop(&mut self) {
        // TODO: Implement the drop method
        println!("Closing file: {}", self.filename);
    }
}

