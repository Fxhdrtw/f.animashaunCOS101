fn main() {
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("array is {:?}", arr);
    println!("array size is {}", arr.len());

    // The iter() function fetches values of all elements in an array
    for val in arr.iter() {
        println!("value is {}", val);
    }
}
