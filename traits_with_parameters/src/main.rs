mod processor;
mod processors;

use processor::TextProcessor;
use processors::{SimpleProcessor, FancyProcessor};

fn main() {
    // Read inputs
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).expect("Failed to read line");
    let text = text.trim();
    
    let mut times_input = String::new();
    std::io::stdin().read_line(&mut times_input).expect("Failed to read line");
    let times: u32 = times_input.trim().parse().expect("Failed to parse times");
    
    let mut max_len_input = String::new();
    std::io::stdin().read_line(&mut max_len_input).expect("Failed to read line");
    let max_len: usize = max_len_input.trim().parse().expect("Failed to parse max_len");
    
    // Create processors
    let simple = SimpleProcessor;
    let fancy = FancyProcessor;
    
    // TODO: Use the processors to transform the text and print the results
    // Print four lines:
    // Simple repeat: {result}
    // Simple truncate: {result}
    // Fancy repeat: {result}
    // Fancy truncate: {result}
    let simple_repeated = simple.repeat(text, times);
    let simple_truncated = simple.truncate(text, max_len);
    let fancy_repeated = fancy.repeat(text, times);
    let fancy_truncated = fancy.truncate(text, max_len);    
    println!("Simple repeat: {}", simple_repeated);
    println!("Simple truncate: {}", simple_truncated);
    println!("Fancy repeat: {}", fancy_repeated);
    println!("Fancy truncate: {}", fancy_truncated);    
}
