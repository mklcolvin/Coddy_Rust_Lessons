mod animals;

use animals::{Speak, Dog, Cat, Bird};

fn main() {
    // Read input for animal names
    let mut dog_name = String::new();
    std::io::stdin().read_line(&mut dog_name).expect("Failed to read line");
    let dog_name = dog_name.trim().to_string();

    let mut cat_name = String::new();
    std::io::stdin().read_line(&mut cat_name).expect("Failed to read line");
    let cat_name = cat_name.trim().to_string();

    let mut bird_name = String::new();
    std::io::stdin().read_line(&mut bird_name).expect("Failed to read line");
    let bird_name = bird_name.trim().to_string();

    // TODO: Create a Vec<Box<dyn Speak>> called zoo
    // Add animals in order: Dog first, then Cat, then Bird

    let zoo: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog { name: dog_name }),
        Box::new(Cat { name: cat_name }),
        Box::new(Bird { name: bird_name }),
    ];

    // TODO: Iterate through the zoo and print each animal's sound

    for animal in &zoo {
        println!("{}", animal.speak());
    }


}
