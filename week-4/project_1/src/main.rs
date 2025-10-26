use std::io;

fn main() {
    // Input values of a, b, and c
    println!("Enter the value of a:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of b:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of c:");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please enter a valid number");

    // Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Check the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root:");
        println!("Root = {}", root);
    } else {
        println!("The equation has no real roots.");
    }
}
