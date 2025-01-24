use std::convert::Infallible;

fn abort() -> ! {
    panic!("This function will never return!");
}

fn check_value(x: i32) -> i32 {
    if x < 0 {
        abort(); // This will never return.
    }
    x
}

// Define a trait for handling the might_fail functionality
trait MightFail {
    fn might_fail(&self) -> Result<i32, Infallible>;
}

// Implement the trait for usize (word type)
impl MightFail for usize {
    fn might_fail(&self) -> Result<i32, Infallible> {
        if *self > 0 {
            Ok(42)
        } else {
            println!("Word type: {} {:p}", self, self);
            abort() // Will not return, hence the never type.
        }
    }
}

// Implement the trait for u8 (byte type)
impl MightFail for u8 {
    fn might_fail(&self) -> Result<i32, Infallible> {
        if *self > 0 {
            Ok(24) // Return a different value for demonstration
        } else {
            println!("Byte type: {} {:p}", self, self);
            abort() // Will not return, hence the never type.
        }
    }
}

fn main() {
    println!("{:x}", 1692134); // Print in hexadecimal
    println!("{}", check_value(5)); // Print the result of check_value

    // Call might_fail with a usize (word type)
    match 3usize.might_fail() {
        Ok(value) => println!("Word Value: {}", value),
        Err(_) => {} // Handle the infallible case (it will never happen)
    }

    // Call might_fail with a u8 (byte type)
    match 0u8.might_fail() {
        Ok(value) => println!("Byte Value: {}", value),
        Err(_) => {} // Handle the infallible case (it will never happen)
    }

    check_value(-20); // This will call abort
    std::process::exit(1); // Exit the program
}
