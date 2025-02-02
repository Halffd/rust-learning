struct Channel {
    users: Vec<String>,
}

impl Channel {
    fn new() -> Self {
        Channel { users: Vec::new() }
    }

    // Sends a vector by taking ownership
    fn send_owned(&mut self, users: Vec<String>) {
        self.users = users; // Ownership transferred here
    }

    // Sends a vector by borrowing
    fn send_borrowed(&mut self, users: &Vec<String>) {
        self.users = users.clone(); // Clone to own a copy
    }

    // Print users using the print_vec function
    fn print_vec(&self) {
        for user in &self.users {
            println!("{}", user);
        }
    }
}

fn main() {
    let mut channel = Channel::new();

    // Create a mutable vector of users
    let users = vec!["Alice".to_string(), "Bob".to_string()];

    // Send the users through the channel by taking ownership
    channel.send_owned(users); // `users` is moved here

    // Uncommenting the following line will cause a compile-time error:
    // println!("{:?}", users); // Error: value moved

    // Create another vector of users to demonstrate borrowing
    let users_borrowed = vec!["Charlie".to_string(), "Diana".to_string()];

    // Send the users through the channel by borrowing
    channel.send_borrowed(&users_borrowed); // Pass a reference to `users_borrowed`

    // Now this works because `users_borrowed` was not moved
    println!("{:?}", users_borrowed); // This will now work

    // Print all users in the channel using the print_vec function
    println!("Users in the channel:");
    channel.print_vec(); // Should print Alice, Bob, Charlie, Diana
}
