fn main() {
    // Example 1: Destructuring a tuple
    let (some_char, some_int) = ('a', 17);
    assert!(some_char == 'a');
    assert!(some_int == 17);
    println!("Destructured values: {} and {}", some_char, some_int);

    // Example 2: Splitting a slice
    let slice = [1, 2, 3, 4, 5];
    let middle = 3; // Assuming we want to split at index 3
    let (l, r) = slice.split_at(middle);
    println!("Left slice: {:?}, Right slice: {:?}", l, r);

    // Example 3: Ignoring a value when destructuring
    let (_, right) = slice.split_at(middle);
    println!("Right slice (ignoring left): {:?}", right);
}

// Function to demonstrate tuple
fn demonstrate_tuple() {
    let example = (42, "Hello");
    let (number, text) = example;
    println!("Number: {}, Text: {}", number, text);
}