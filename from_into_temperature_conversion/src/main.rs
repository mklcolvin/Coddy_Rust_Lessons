mod temperature;

use temperature::{Celsius, Fahrenheit};

fn main() {
    // Read input
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let celsius1: f64 = input1.trim().parse().expect("Invalid number");
    
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let celsius2: f64 = input2.trim().parse().expect("Invalid number");
    
    // TODO: Create a Celsius value from celsius1
    let celsius1 = Celsius(celsius1);
    
    // TODO: Convert it to Fahrenheit using Fahrenheit::from()
    let fahrenheit1 = Fahrenheit::from(celsius1);
    
    // TODO: Create another Celsius value from celsius2
    let celsius2 = Celsius(celsius2);
    
    // TODO: Convert it to Fahrenheit using .into()
    let fahrenheit2: Fahrenheit = celsius2.into();
    
    // TODO: Print the results in the format:
    // From: {celsius1}C = {fahrenheit1}F
    // Into: {celsius2}C = {fahrenheit2}F
    println!("From: {:.0}C = {:.0}F", celsius1.value() as i64, fahrenheit1.value() as i64);
    println!("Into: {:.0}C = {:.0}F", celsius2.value() as i64, fahrenheit2.value() as i64);
}
