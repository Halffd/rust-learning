fn main() {
    let command = "Hello, World!";

    match command {
        "Hello, World!" => {
            println!("Hello to you too!");
        }
        "Goodbye, World!" => {
            println!("See you later!");
        }
        _ => {
            println!("No match found!");
        }
    }
}