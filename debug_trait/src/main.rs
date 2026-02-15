mod inventory;

use inventory::Item;

fn main() {
    // Read input
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    let mut quantity_str = String::new();
    std::io::stdin().read_line(&mut quantity_str).expect("Failed to read line");
    let quantity: u32 = quantity_str.trim().parse().expect("Failed to parse quantity");
    
    // TODO: Create an Item instance with the given name and quantity
    let item = Item { name: name, quantity: quantity };
    
    // TODO: Print the item using the debug formatter {:?}
    println!("{:?}", item);

}
