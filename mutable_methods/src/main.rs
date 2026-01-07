use std::io;

// TODO: Define the BankAccount struct here

struct BankAccount {
    balance: i32,
}
// TODO: Add the implementation block for BankAccount with deposit, withdraw, and get_balance methods

impl BankAccount {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }

    fn get_balance(&self) -> i32 {
        self.balance
    }
}

fn main() {
    let mut input = String::new();
    
    // Read initial balance
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let initial_balance: i32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Read deposit amount
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let deposit_amount: i32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Read withdrawal amount
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let withdraw_amount: i32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create a mutable BankAccount instance with initial_balance
    // TODO: Call deposit with deposit_amount
    // TODO: Call withdraw with withdraw_amount
    // TODO: Print the final balance using get_balance

    let mut my_bankaccount = BankAccount {balance: initial_balance};
    my_bankaccount.deposit(deposit_amount);
    my_bankaccount.withdraw(withdraw_amount);
    let my_balance = my_bankaccount.get_balance();
    println!("{}", my_balance);

}