// Function to swap two values using mutable references
fn troca(x: &mut i32, y: &mut i32) {
    let auxiliar = *x; // Dereference x to get its value
    *x = *y;          // Assign y's value to x
    *y = auxiliar;    // Assign the original x value to y
}

fn main() {
    // Example 1: Swapping two integers
    let mut a = 10;
    let mut b = 5;
    println!("Before swap: a = {}, b = {}", a, b);
    troca(&mut a, &mut b); // Pass mutable references
    println!("After swap: a = {}, b = {}", a, b);

    // Example 2: Swapping negative numbers
    let mut c = -3;
    let mut d = 7;
    println!("Before swap: c = {}, d = {}", c, d);
    troca(&mut c, &mut d);
    println!("After swap: c = {}, d = {}", c, d);

    // Example 3: Swapping zeros
    let mut e = 0;
    let mut f = 0;
    println!("Before swap: e = {}, f = {}", e, f);
    troca(&mut e, &mut f);
    println!("After swap: e = {}, f = {}", e, f);

    // Example 4: Swapping large numbers
    let mut g = 123456789;
    let mut h = 987654321;
    println!("Before swap: g = {}, h = {}", g, h);
    troca(&mut g, &mut h);
    println!("After swap: g = {}, h = {}", g, h);
}