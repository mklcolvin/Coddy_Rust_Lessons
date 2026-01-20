mod describable;
mod items;

use describable::Describable;
use items::{Book, Movie};

fn main() {
    // Read inputs
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim().to_string();
    
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    // TODO: Create a Book instance with the title
    let book = Book {title: title};
    
    // TODO: Create a Movie instance with the name
    let movie = Movie {name: name};

    // TODO: Print the description of the book
    println!("Book: {}", book.title);

    // TODO: Print the description of the movie
    println!("Film: {}", movie.name);
    
}
