use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel for sending and receiving messages
    let (tx, rx) = mpsc::channel();

    // Create a mutable Vec to store user names
    let mut users = Vec::new();

    // Simulate adding some usernames to the vector
    users.push("Alice".to_string());
    users.push("Bob".to_string());

    // Spawn two threads to add new users to the vector
    let t1 = {
        let tx = tx.clone();
        let users = users.clone();
        thread::spawn(move || {
            // Add a new user to the vector
            users.push("Charlie".to_string());

            // Send the updated vector over the channel
            tx.send(users).unwrap();
        })
    };

    let t2 = {
        let tx = tx.clone();
        let users = users.clone();
        thread::spawn(move || {
            // Add a new user to the vector
            users.push("David".to_string());

            // Send the updated vector over the channel
            tx.send(users).unwrap();
        })
    };

    // Receive the users on the main thread
    let received_users1 = rx.recv().unwrap();
    let received_users2 = rx.recv().unwrap();
    println!("Received users1: {:?}", received_users1);
    println!("Received users2: {:?}", received_users2);

    // Wait for the spawned threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the original users vector
    println!("Original users: {:?}", users);
}
