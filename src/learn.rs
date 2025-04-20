// Struct definition for demonstration
struct Person {
    nickname: &'static str,
}

// Function returning a struct instance
fn get_some_struct() -> Person {
    Person {
        nickname: "fasterthanlime",
    }
}

// Function to simulate a fair dice roll based on feeling lucky
fn fair_dice_roll(feeling_lucky: bool) -> i32 {
    // Using match statement
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

// Alternate implementation using if statement
fn fair_dice_roll_if(feeling_lucky: bool) -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}

fn main() {
    // Example of accessing tuple fields
    let a = (10, 20);
    println!("Accessing tuple field: {}", a.0); // Output: 10

    // Example of accessing fields of a struct
    let amos = get_some_struct();
    println!("Accessing struct field: {}", amos.nickname); // Output: "fasterthanlime"

    // Fair dice roll examples
    let lucky_roll = fair_dice_roll(true);
    println!("Fair dice roll (lucky): {}", lucky_roll); // Output: 6

    let unlucky_roll = fair_dice_roll_if(false);
    println!("Fair dice roll (unlucky): {}", unlucky_roll); // Output: 4
}