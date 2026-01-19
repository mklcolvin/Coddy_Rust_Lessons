mod pet;

use pet::Pet;

fn main() {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

//    let mut energy_value = String::new();
//    std::io::stdin().read_line(&mut energy_value).expect("Failed to read line");
//    let energy_value: u32 = energy_value.trim().parse::<u32>().unwrap();

//    let mut hunger_value = String::new();
//    std::io::stdin().read_line(&mut hunger_value).expect("Failed to read line");
//    let hunger_value: u32 = hunger_value.trim().parse::<u32>().unwrap();

    // TODO: Create a new Pet with the given name

    let mut pet = Pet::new(name);

    // Set the starting hunger_value
//    let _result = pet.set_hunger(hunger_value);

    // Set the starting energy_value
//    let _result = pet.set_energy(energy_value);

// Print the pet's status
    pet.status();

// Play with the pet
    pet.play();
    pet.play();

//   println!("After playing:");
//    println!("Energy: {}, Hunger: {}", pet.energy(), pet.hunger());


// Get the pet's energy and hunger
//    let _result = pet.energy();
//    let _result = pet.hunger();
//    println!("{}'s starting energy: {}", pet.name(), energy);
//    println!("{}'s starting hunger: {}", pet.name(), hunger);

//Feed the pet 30 units of food
    pet.feed();

// Print the After Activities separator line
    println!("--- After Activities ---");
        
//    print!("Hunger: {}", pet.hunger());

// Print the pet's status again
    pet.status();

//End of Line
}
