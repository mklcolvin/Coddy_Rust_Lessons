mod traffic;

use traffic::TrafficLight;

fn main() {
    // Read input for durations
    let mut red_input = String::new();
    std::io::stdin().read_line(&mut red_input).expect("Failed to read line");
    let red_duration: u32 = red_input.trim().parse().expect("Invalid number");

    let mut yellow_input = String::new();
    std::io::stdin().read_line(&mut yellow_input).expect("Failed to read line");
    let yellow_duration: u32 = yellow_input.trim().parse().expect("Invalid number");

    let mut green_input = String::new();
    std::io::stdin().read_line(&mut green_input).expect("Failed to read line");
    let green_duration: u32 = green_input.trim().parse().expect("Invalid number");

    // TODO: Create instances of each TrafficLight variant with the input durations

    let red_light = TrafficLight::Red;
    println!("{}", red_light.status(red_duration));

    let yellow_light = TrafficLight::Yellow;
    println!("{}", yellow_light.status(yellow_duration));

    let green_light = TrafficLight::Green;
    println!("{}", green_light.status(green_duration));

    // TODO: Call the status method on each light and print the result


}
