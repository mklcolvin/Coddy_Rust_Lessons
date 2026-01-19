use std::io;

fn calculate_average (list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}


fn main() {
    // Read the number of grades
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read line");
    let count: i32 = count_input.trim().parse().expect("Invalid number");
    
    // Create a vector to store grades
    let mut grades: Vec<i32> = Vec::new();
    
    // Read each grade
    for _ in 0..count {
        let mut grade_input = String::new();
        io::stdin().read_line(&mut grade_input).expect("Failed to read line");
        let grade: i32 = grade_input.trim().parse().expect("Invalid number");
        grades.push(grade);
    }
    
    // TODO: Write your code below
    // Calculate average, find highest, find lowest, count passing and failing grades
    
    let average = calculate_average(&grades) as i32;
    let highest = grades.iter().max().unwrap();
    let lowest = grades.iter().min().unwrap();
    let passing_count = grades.iter().filter(|&&x| x >= 70).count();
    let failing_count = grades.iter().filter(|&&x| x < 70).count();
    
    // Output the results in the required format
    println!("Average: {}", average);
    println!("Highest: {}", highest);
    println!("Lowest: {}", lowest);
    println!("Passing: {}", passing_count);
    println!("Failing: {}", failing_count);
}