fn main() {
    let toshiba_qty: u32 = 2;
    let toshiba_amount: f64 = 450_000.00;

    let mac_qty: u32 = 1;
    let mac_amount: f64 = 1_500_000.00;

    let hp_qty: u32 = 3;
    let hp_amount: f64 = 750_000.00;

    let dell_qty: u32 = 3;
    let dell_amount: f64 = 2_850_000.00;

    let acer_qty: u32 = 1;
    let acer_amount: f64 = 250_000.00;

    let sum = toshiba_amount + mac_amount + hp_amount + dell_amount + acer_amount;
    println!("Total Sales Amount is {}", sum);

    let total_qty = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
    let average = sum / total_qty as f64;
    println!("Average Sales Amount per item is {}", average);
}