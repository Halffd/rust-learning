//! A file I/O utility crate

pub mod file {
    use std::fs;
    use std::io;
    use std::path::Path;

    /// Reads content from a file
    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
        fs::read_to_string(path)
    }

    /// Writes content to a file
    pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
        fs::write(path, contents)
    }

    /// Processes file content with min value
    pub fn process_with_min<P: AsRef<Path>>(path: P, a: usize, b: usize) -> io::Result<String> {
        let content = read(path)?;
        let least = std::cmp::min(a, b);
        Ok(content.to_uppercase().repeat(least))
    }

    /// Numeric processing function
    pub fn numeric_operation(least: i32) -> i32 {
        println!("Processing value: {}", least);
        least * 2
    }
}

fn main() -> std::io::Result<()> {
    // File operations
    let path = "example.txt";
    
    // Write to file
    file::write(path, "Hello, file module!")?;
    println!("File created successfully");

    // Read from file
    let content = file::read(path)?;
    println!("File content: {}", content);

    // Process with min value
    let processed = file::process_with_min(path, 3, 8)?;
    println!("Processed content: {}", processed);

    // Numeric operation
    let result = file::numeric_operation(3);
    println!("Doubled value: {}", result);

    Ok(())
}