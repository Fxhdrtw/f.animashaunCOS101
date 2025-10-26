use std::io;

fn main() {
    // Input experience
    println!("Is the employee experienced? (yes/no):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Input age
    println!("Enter the age of the employee:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    // Determine incentive
    if experience == "yes" {
        if age >= 40 {
            println!("The incentive of the employee is ₦1,560,000 per year.");
        } else if age >= 30 && age < 40 {
            println!("The incentive of the employee is ₦1,480,000 per year.");
        } else if age < 28 {
            println!("The incentive of the employee is ₦1,300,000 per year.");
        } else {
            println!("The incentive of the employee is ₦1,350,000 per year (default for experienced employees between 28–29).");
        }
    } else {
        println!("The incentive of the employee is ₦100,000 per year.");
    }
}
