mod mybox;

use mybox::MyBox;

fn main() {
    // Read inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let initial_int: i32 = input1.trim().parse().expect("Invalid integer");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let replacement_int: i32 = input2.trim().parse().expect("Invalid integer");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let string_value = input3.trim().to_string();

    // TODO: Create a MyBox with the initial integer
    let mut mybox = MyBox { contents: initial_int };


    // TODO: Peek at its contents and print: "Integer box contains: {value}"
    println!("Integer box contains: {}", mybox.peek());

    // TODO: Replace the contents with the replacement integer
    mybox.replace (replacement_int);

    // TODO: Peek again and print: "After replace: {value}"
    println!("After replace: {}", mybox.peek());
    
    // TODO: Create a second MyBox with the string value
    let stringbox = MyBox { contents: string_value };

    // TODO: Peek at the string box and print: "String box contains: {value}"
    println!("String box contains: {}", stringbox.peek());

}
