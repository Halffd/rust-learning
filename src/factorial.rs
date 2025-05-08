fn factorial(n: u32) -> u32 {
    println!("Calling factorial({})", n);
    if n == 0 {
        println!("Reached base case: factorial(0) = 1");
        1
    } else {
        let result = n * factorial(n - 1);
        println!("Computed factorial({}) = {}", n, result);
        result
    }
}

fn main() {
    let result = factorial(5);
    println!("Result: {}", result);
}
