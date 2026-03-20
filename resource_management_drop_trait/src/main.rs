mod file_handle;

use file_handle::FileHandle;

// TODO: Implement the process_file function
// It should:
// 1. Create a local scope (using curly braces {})
// 2. Inside that scope, create a FileHandle using FileHandle::new()
// 3. Print the processing message: "Processing {filename}..."
// 4. When the scope ends, Drop will automatically be called
fn process_file(filename: &str) {
    // TODO: Implement this function
    {
        let my_file = FileHandle::new(filename);
        println!("Processing {}...", filename);
    }   
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let filename = input.trim();
    
    process_file(filename);
}
