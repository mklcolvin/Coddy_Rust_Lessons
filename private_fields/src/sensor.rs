// TODO: Define a public TemperatureSensor struct
// - location: String (private)
// - reading: f64 (private)

// TODO: Implement the TemperatureSensor struct
pub struct TemperatureSensor {
    location: String,
    reading: f64,
}

impl TemperatureSensor {
    // TODO: Create a public 'new' constructor that:
    // - Takes a location (String) and reading (f64)
    // - Prints: "Sensor created for: {location}"
    // - Returns the new TemperatureSensor

    pub fn new(my_location: String, my_reading: f64) -> TemperatureSensor {
        println!("Sensor created for: {}", my_location);
        TemperatureSensor { location: my_location, reading: my_reading }
    }    
}
