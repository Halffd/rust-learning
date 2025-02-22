fn main() { 
    // Use the constant to prevent dead code warning
    println!("Default path: {}", DEFAULT_FILE_PATH);
    
    underscore_demo();
    type_placeholder();
    
    // Demonstrate User struct usage
    let _user = User { id: 1, username: "test".to_string() };
}

// Add allow to suppress dead code warnings for demonstration
#[allow(dead_code)]
const DEFAULT_FILE_PATH: &str = "src/server/server.rs";

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    id: i32,
    username: String,
}

// Corrected error enum
#[derive(Debug)]
pub enum DatabaseError {
    SqlxError(()),  // Changed to unit type to suppress warning
    InvalidCredentials,
}

#[derive(Debug)]
pub struct Database;  // Added Database struct definition

// Implement used method
impl Database {
    #[allow(dead_code)]
    pub fn get_user(&self, username: &str) -> Result<Option<User>, DatabaseError> {
        // Implementation would go here
        Ok(None)
    }
}

// Snake case variable name
fn snake_case_demo() {
    let iter = 42;
    println!("Iter value: {}", iter);
}

// Single underscore_demo implementation
fn underscore_demo() {
    // 1. Ignoring function parameters
    fn ignore_parameter(_: i32) {
        println!("Parameter ignored");
    }

    ignore_parameter(42);

    // 2. Ignoring parts of a tuple
    let (a, _, c) = (1, 2, 3);
    println!("First: {}, Third: {}", a, c);

    // 3. Ignoring match arms
    let value = 5;
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }

    // 4. Ignoring unused variables
    let _unused = 42;
    let _ = "temporary value";
    
    // 7. Explicitly ignoring constant values
    let _ = 42;
    println!("Constant ignored");

    // 8. Discarding function return values
    let _ = get_thing();
}

// Add missing pattern_ignoring function
fn pattern_ignoring() {
    println!("Pattern ignoring demo");
}

fn type_placeholder() {
    let _numbers: Vec<_> = vec![1, 2, 3].into_iter().collect();

    fn generic_function<T>(_: T) {
        println!("Generic function called");
    }

    generic_function::<i32>(5);
}

fn get_thing() -> i32 {
    println!("get_thing() called!");
    42
}