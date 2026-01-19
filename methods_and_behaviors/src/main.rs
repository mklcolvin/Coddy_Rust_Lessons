use std::io;

// TODO: Define the Player struct with name (String) and score (u32) fields
struct Player {
    name: String,
    score: u32,
}

// TODO: First impl block - Constructor and query methods (new, get_score)

impl Player {
    fn new(new_name: String) -> Player {
        Player {
            name: new_name,
            score: 0,
        }
    }

    fn get_score(&self) -> u32 {
        return self.score
    }
}


// TODO: Second impl block - Score modification methods (add_points, reset_score)

impl Player {

    fn add_points(&mut self, points: u32) {
        self.score += points;
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }
}

fn main() {
    let mut input = String::new();
    
    // Read player name
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim().to_string();
    
    // Read first round points
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_round: u32 = input.trim().parse().expect("Invalid number");
    
    // Read second round points
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_round: u32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create a player using Player::new
    // TODO: Add first round points and print the score
    // TODO: Reset the score
    // TODO: Add second round points and print the final score

    let mut my_player = Player::new(name);
    my_player.add_points(first_round);
    println!("{}", my_player.get_score());
    my_player.reset_score();
    my_player.add_points(second_round);
    println!("{}", my_player.get_score());

// End of Line
}
