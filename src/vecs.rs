use std::io::{self, Write};

fn main() {
    // Option examples
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine - returns 128
    
    let o2: Option<i32> = None;
    // o2.unwrap(); // This would panic if uncommented!

    // Vector examples
    let mut v1 = Vec::new();
    v1.push(1);  // v1: Vec<i32>
    
    let mut v2 = Vec::new();
    v2.push(false);  // v2: Vec<bool>

    // IO example
    io::stdout()
        .lock()
        .write_all(b"Hello there!\n")
        .unwrap();
}