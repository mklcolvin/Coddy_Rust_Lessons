mod smart_box;

use smart_box::SmartBox;

fn main() {
    // Read inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number: i32 = input1.trim().parse().expect("Failed to parse number");
    
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let text = input2.trim().to_string();
    
    // TODO: Create a SmartBox<i32> holding the number
    let smart_box_number = SmartBox::new(number);

    // TODO: Create a SmartBox<String> holding the text
    let smart_box_text = SmartBox::new(text);

    // TODO: Print the values using the get method
    // Format: "Value: {value}"
    println!("Value: {}", smart_box_number.get());
    println!("Value: {}", smart_box_text.get());

    // The Drop messages will appear automatically when main ends
    drop(smart_box_number);
    drop(smart_box_text);

}
