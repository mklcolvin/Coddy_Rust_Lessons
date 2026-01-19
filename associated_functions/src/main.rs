use std::io;

// TODO: Define the Book struct with title (String) and pages (u32) fields

struct Book {
    title: String,
    pages: u32,
}



// TODO: Add an impl block for Book with:
// - An associated function `new` that takes a title and returns a Book with pages set to 0
// - A method `info` that returns a String in the format "{title} has {pages} pages"

impl Book {
    fn new(new_title: String) -> Book {
        Book {
            title: new_title,
            pages: 0,
        }
    }

    fn info(&self) -> String {
        let my_string: String = format!("{} has {} pages", self.title, self.pages).to_string();
        return my_string;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let title = input.trim().to_string();

    // TODO: Use Book::new to create a book with the provided title
    // TODO: Print the result of calling the info method

    let my_book = Book::new(title);
    println!("{}", my_book.info());

}