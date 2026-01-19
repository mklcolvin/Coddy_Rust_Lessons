use std::io;

// TODO: Define the Cat struct with 'name' (String) and 'lives' (u8) fields

struct Cat {
    name: String,
    lives: u8,
}
// TODO: Add an implementation block for Cat with a 'meow' method

impl Cat {
    fn meow (&self) {
        println!("{} says meow!", self.name);
    }
}
fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    let mut lives_input = String::new();
    io::stdin().read_line(&mut lives_input).expect("Failed to read line");
    let lives: u8 = lives_input.trim().parse().expect("Failed to parse lives");
    
    // TODO: Create a Cat instance and call the meow method
    
    let my_cat = Cat {
        name: String::from(name),
        lives: lives,
    };

    my_cat.meow();
}