mod container;

use container::Container;

fn main() {
    // Read the three inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let int_value: i32 = input1.trim().parse().expect("Failed to parse integer");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let float_value: f64 = input2.trim().parse().expect("Failed to parse float");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let string_value = input3.trim().to_string();

    // TODO: Create three containers using Container<T>
    // 1. An integer container holding int_value
    // 2. A float container holding float_value
    // 3. A string container holding string_value

    let int_container = Container { item: int_value };
    let float_container = Container { item: float_value };
    let string_container = Container { item: string_value };

    // TODO: Print each container's contents in the required format
    // Integer container: {value}
    // Float container: {value}
    // String container: {value}

    println!("Integer container: {}", int_container.item);
    println!("Float container: {}", float_container.item);
    println!("String container: {}", string_container.item);    
    
}