use std::process::Command;

fn main() {
    // Create a new Command instance for the "http" program
    let mut cmd = Command::new("http");

    // Add arguments to the command
    cmd.arg("get");
    cmd.arg("localhost");

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            // Print the output of the command
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            // Print the error if the command fails
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
