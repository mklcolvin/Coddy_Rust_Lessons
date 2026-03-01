mod vehicle;

use vehicle::{Rentable, Car, Bike};

fn main() {
    // Read input
    let mut car_model = String::new();
    std::io::stdin().read_line(&mut car_model).expect("Failed to read line");
    let car_model = car_model.trim().to_string();
    
    let mut bike_brand = String::new();
    std::io::stdin().read_line(&mut bike_brand).expect("Failed to read line");
    let bike_brand = bike_brand.trim().to_string();
    
    // TODO: Create a Box<dyn Rentable> variable that holds a Car
    // Print its rental info using the rental_info() method
    let mut rentable: Box<dyn Rentable> = Box::new(Car { model: car_model });
    println!("{}", rentable.rental_info());
    
    // TODO: Reassign the same variable to hold a Bike
    // Print its rental info again
    
    rentable = Box::new(Bike { brand: bike_brand });
    println!("{}", rentable.rental_info());
    
}
