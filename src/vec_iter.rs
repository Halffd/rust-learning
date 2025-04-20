fn main() {
    {
        // Create a vector with elements 1 through 8
        let x = vec![1, 2, 3, 4, 5, 6, 7, 8];

        // Process the vector using iterators and closures
        let result = x
            .iter() // Create an iterator over the vector
            .map(|x| x + 3) // Add 3 to each element
            .fold(0, |x, y| x + y); // Sum all the elements

        // Print the result
        println!("The result is: {}", result);
    }
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = x
        .iter()
        .map(|&x| x + 3) // Dereference x to get the value
        .fold(0, |x, y| x + y);
    println!("The result is: {}", result);
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
}
