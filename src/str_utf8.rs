fn main() {
    // Example 1: Valid UTF-8 bytes (watermelon emoji 🍉)
    let s1 = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s1); // Output: Ok("🍉")

    // Example 2: Invalid UTF-8 bytes
    let s2 = std::str::from_utf8(&[195, 255]);
    println!("{:?}", s2); // Output: Err(Utf8Error...)
    
    // Example 3: Using unwrap() with valid bytes
    let s3 = std::str::from_utf8(&[240, 159, 141, 137]).unwrap();
    println!("{:?}", s3); // Output: "🍉"

    // Example 4: Using expect() with invalid bytes (would panic)
    // let s4 = std::str::from_utf8(&[195, 255]).expect("valid utf-8");

    // Example 5: Pattern matching
    let melon = &[240, 159, 141, 137];
    match std::str::from_utf8(melon) {
        Ok(s) => println!("Match: {}", s), // Prints 🍉
        Err(e) => panic!("{}", e),
    }

    // Example 6: if let syntax
    let melon = &[240, 159, 141, 137];
    if let Ok(s) = std::str::from_utf8(melon) {
        println!("if let: {}", s); // Prints 🍉
    }
}