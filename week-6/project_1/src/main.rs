use std::io;

fn main() {
    println!("==================== MENU ====================");
    println!("P = Pounded Yam / Edinkaiko Soup  - ₦3,200");
    println!("F = Fried Rice & Chicken          - ₦3,000");
    println!("A = Amala & Ewedu Soup            - ₦2,500");
    println!("E = Eba & Egusi Soup              - ₦2,000");
    println!("W = White Rice & Stew             - ₦2,500");
    println!("==============================================");

    let mut food_type = String::new();
    let mut quantity_input = String::new();

    // Get food type
    println!("Enter food type (P, F, A, E, W):");
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    // Get quantity
    println!("Enter quantity:");
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    let quantity: i32 = quantity_input.trim().parse().expect("Please enter a number");

    // Determine price
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid selection!");
            return;
        }
    };

    // Calculate total
    let mut total = price * quantity;

    // Apply 5% discount if total > 10,000
    if total > 10000 {
        let discount = total as f64 * 0.05;
        total = (total as f64 - discount) as i32;
        println!("\nA 5% discount has been applied!");
    }

    println!("\n==============================================");
    println!("Food Type: {}", food_type);
    println!("Quantity: {}", quantity);
    println!("Total Amount to Pay: ₦{}", total);
    println!("==============================================");
}
