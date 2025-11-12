use std::io;

fn main() {
    println!("\nMTH 101: Shape Calculator");
    println!("1. Trapezium  2. Rhombus  3. Parallelogram  4. Cube  5. Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: i32 = choice.trim().parse().unwrap();

    match choice {
        1 => {
            let (h, b1, b2) = (get("height"), get("base1"), get("base2"));
            println!("Area of Trapezium = {}", h / 2.0 * (b1 + b2));
        }
        2 => {
            let (d1, d2) = (get("diagonal1"), get("diagonal2"));
            println!("Area of Rhombus = {}", 0.5 * d1 * d2);
        }
        3 => {
            let (b, a) = (get("base"), get("altitude"));
            println!("Area of Parallelogram = {}", b * a);
        }
        4 => {
            let s = get("length of side");
            println!("Area of Cube = {}", 6.0 * s.powi(2));
        }
        5 => {
            let (r, h) = (get("radius"), get("height"));
            println!("Volume of Cylinder = {}", std::f64::consts::PI * r.powi(2) * h);
        }
        _ => println!("Invalid choice!"),
    }
}

fn get(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("Enter {}:", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
