// Custom Result type implementation
enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(_) => panic!("Called unwrap on Err value"),
        }
    }
}

// Function definitions MUST come before main
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Result::Ok(n),
        Err(e) => Result::Err(format!("Failed to parse '{}': {}", s, e)),
    }
}

fn divide(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        Result::Err("Cannot divide by zero".to_string())
    } else {
        Result::Ok(a as f64 / b as f64)
    }
}

// Main function comes LAST
fn main() {
    let numbers = ["42", "hello", "100"];

    for num in &numbers {
        match parse_number(num) {
            Result::Ok(n) => println!("Parsed number: {}", n),
            Result::Err(e) => println!("Error: {}", e),
        }
    }

    let operations = [(10, 2), (5, 0), (8, 40)];
    for (a, b) in &operations {
        match divide(*a, *b) {
            Result::Ok(result) => println!("{} / {} = {:.2}", a, b, result),
            Result::Err(e) => println!("{} / {}: {}", a, b, e),
        }
    }

    let valid = parse_number("123").unwrap();
    println!("Unwrapped value: {}", valid);
}
