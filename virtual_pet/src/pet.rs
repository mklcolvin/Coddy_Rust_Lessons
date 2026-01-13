// TODO: Define a public Pet struct with private fields:
// - name: String
// - energy: u32
// - hunger: u32

pub struct Pet {
    name: String,
    energy: u32,
    hunger: u32,
}

// TODO: Implement the following for Pet:
// - A public associated function `new` that takes a name (String)
//   and returns a Pet with energy = 100 and hunger = 0
// - A public method `name` that returns a reference to the pet's name (&String)

impl Pet {
    pub fn new(name: String) -> Self {
        Pet {name, energy: 100, hunger: 0 }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn feed(&mut self) {
        // Decrease hunger by food_amount, ensuring it doesn't go below 0   
        if self.hunger < 30 {
            self.hunger = 0;
        } else {
            self.hunger -= 30;
        }
         println!("{} enjoyed the meal!", self.name);
    }

    pub fn set_hunger(&mut self, do_hunger: u32) {
        self.hunger = do_hunger;
    }   

    pub fn set_energy(&mut self, do_energy: u32) {
        self.energy = do_energy;
    }
    
    pub fn hunger(&self) -> u32  {
        println!("Hunger: {}", self.hunger);
        self.hunger
        
    } 

     pub fn energy(&self) -> u32  {
        println!("Energy: {}", self.energy);
        self.energy

    } 

    pub fn play (&mut self) {
        if self.energy >= 20 {
            self.energy -= 20} 
            else {
                self.energy = 0 
        }   

        if self.hunger >= 85 {
            self.hunger =100 }
            else {
                self.hunger += 15;
        }

        println!("{} had fun playing!", self.name);
    }   

    pub fn status(&self) {
        let mut mood: String = String::new();
        println!("--- Status Report ---");
        println!("Name: {}", self.name);
        println!("Energy: {}", self.energy);
        println!("Hunger: {}", self.hunger);

        // Prints the pet's mood based on energy and hunger

        if self.energy >= 50 && self.hunger <= 50 {
            mood = String::from("happy");
        } else {
           mood = String::from("tired");
        } 
        
        

        println!("Mood: {}", mood);

    }  


}
