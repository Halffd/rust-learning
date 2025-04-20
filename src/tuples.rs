fn main() {
    // Example 1: Implicit type annotation
    let pair = ('a', 17);
    println!("Implicitly typed tuple:");
    println!("First element: {}", pair.0); // Output: this is 'a'
    println!("Second element: {}", pair.1); // Output: this is 17

    // Example 2: Explicit type annotation
    let explicit_pair: (char, i32) = ('a', 17);
    println!("\nExplicitly typed tuple:");
    println!("First element: {}", explicit_pair.0); // Output: this is 'a'
    println!("Second element: {}", explicit_pair.1); // Output: this is 17

    // Example 3: Tuple with multiple types
    let mixed_tuple = (3.14, "Hello", true);
    println!("\nMixed type tuple:");
    println!("First element: {}", mixed_tuple.0); // Output: 3.14
    println!("Second element: {}", mixed_tuple.1); // Output: Hello
    println!("Third element: {}", mixed_tuple.2); // Output: true

    // Example 4: Nested tuples
    let nested_tuple = (1, (2, 3));
    println!("\nNested tuple:");
    println!("First element: {}", nested_tuple.0); // Output: 1
    println!("Second element: ({}, {})", nested_tuple.1 .0, nested_tuple.1 .1); // Output: (2, 3)

    // Example 5: Destructuring tuples
    let (x, y) = (10, 20);
    println!("\nDestructured tuple:");
    println!("x: {}, y: {}", x, y); // Output: x: 10, y: 20

    // Example 6: Returning a tuple from a function
    let result = calculate(5, 10);
    println!("\nFunction returning a tuple:");
    println!("Sum: {}, Product: {}", result.0, result.1); // Output: Sum: 15, Product: 50
}

// Function that returns a tuple
fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}