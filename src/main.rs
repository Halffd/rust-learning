fn main() {
    let arr = ["a", "b"];

    // Attempt to access an out-of-bounds index
    let index = 100;

    // Using .get() for safe access
    match arr.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds.", index),
    }
}
