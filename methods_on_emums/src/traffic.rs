#[derive(Debug)]

// TODO: Define a public TrafficLight enum with three variants:
// - Red(u32)
// - Yellow(u32)
// - Green(u32)

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}


// TODO: Implement the TrafficLight enum with a status method
// The status method should return a String with the appropriate message:
// - Red: "Stop for {seconds} seconds"
// - Yellow: "Caution for {seconds} seconds"
// - Green: "Go for {seconds} seconds"
// Use match to handle each variant

impl TrafficLight {
    pub fn status(&self, seconds: u32) -> String {
        match self {
            TrafficLight::Red => format!("Stop for {} seconds", seconds),
            TrafficLight::Yellow  => format!("Caution for {} seconds", seconds), 
            TrafficLight::Green  => format!("Go for {} seconds", seconds),
        }
    }
}
