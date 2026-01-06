mod sensor;

use sensor::TemperatureSensor;

fn main() {
    // Read input
    let mut location = String::new();
    std::io::stdin().read_line(&mut location).expect("Failed to read line");
    let location = location.trim().to_string();
    
    let mut reading_str = String::new();
    std::io::stdin().read_line(&mut reading_str).expect("Failed to read line");
    let reading: f64 = reading_str.trim().parse().expect("Failed to parse reading");
    
    // TODO: Create a TemperatureSensor using the constructor
    // The new function will handle printing the confirmation message
    
    let new_sensor=TemperatureSensor::new(location, reading);
    
}
