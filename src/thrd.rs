use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel for thread communication
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread and move the receiver into it
    let handle = thread::spawn(move || {
        // Wait for message from main thread
        match receiver.recv() {
            Ok(msg) => {
                if msg == "join a rust discord server" {
                    println!("/thread");
                }
            }
            Err(e) => eprintln!("Thread error: {}", e),
        }
    });

    // Send message from main thread
    sender
        .send("join a rust discord server")
        .expect("Failed to send message");

    // Wait for the thread to finish
    handle.join().expect("Failed to join thread");
}
