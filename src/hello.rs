use std::fmt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greetings = vec![
        "Hello, world!".to_string(),
        "¡Hola Mundo!".to_string(),
        "Γειά σου Κόσμε!".to_string(),
        "Привет, мир!".to_string(),
        "こんにちは世界!".to_string(),
    ];

    println!();
    for greeting in &greetings {
        println!("{}", greeting);
    }

    Ok(())
}

