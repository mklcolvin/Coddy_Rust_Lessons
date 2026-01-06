// TODO: Define a public Command enum with three variants:
// - Say: holds a single String (tuple syntax)
// - Move: holds named fields 'direction' (String) and 'steps' (u32)
// - Calculate: holds two i32 values (tuple syntax)

pub enum Command {
    // TODO: Define the Say variant
    Say(String),
    // TODO: Define the Move variant with named fields
    Move { direction: String, steps: u32 },
    // TODO: Define the Calculate variant
    Calculate(i32, i32),
}

impl Command {
    // Implement the execute method that returns a String
    // Use match to destructure each variant and return the appropriate message:
    // - Say: "Saying: {message}"
    // - Move: "Moving {steps} steps {direction}"
    // - Calculate: "{a} + {b} = {sum}"

    pub fn execute(&self) -> String {
        match self {
            Command::Say(message) => format!("Saying: {}", message),
            Command::Move { direction, steps } => format!("Moving {} steps {}", steps, direction),
            Command::Calculate(first, second) => {
                let sum = first + second;
                format!("{} + {} = {}", first, second, sum)
            }
        }
    }
}
