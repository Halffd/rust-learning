use std::env;
use std::process::Command;
use glob::glob;
use std::path::{Path, PathBuf};
use std::fs;

mod server;

const DEFAULT_FILE_PATH: &str = "src/server/server.rs";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Create build directory if it doesn't exist
    fs::create_dir_all("build").expect("Failed to create build directory");

    if args.len() < 2 {
        // Run the default server
        if let Err(e) = server::server::start_server().await {
            eprintln!("Server error: {}", e);
        }
        return;
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_help();
            list_rust_files();
        }
        file_path => {
            run_rust_file(file_path);
        }
    }
}

fn list_rust_files() {
    println!("\nAvailable Rust files:");
    match glob("**/*.rs").expect("Failed to read glob pattern") {
        Iter => {
            for entry in Iter {
                match entry {
                    Ok(path) => println!("  {}", path.display()),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}

fn run_rust_file(file_path: &str) {
    let path = PathBuf::from(file_path);
    
    if !path.exists() {
        eprintln!("Error: File '{}' not found", file_path);
        return;
    }
    
    if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
        eprintln!("Error: File must be a Rust file (.rs extension)");
        return;
    }

    println!("Compiling and running: {}", file_path);
    
    // Generate executable name from the source file name
    let executable_name = path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|name| format!("build/{}{}", name, env::consts::EXE_SUFFIX))
        .expect("Failed to generate executable name");

    // Compile the file using rustc
    let output = Command::new("rustc")
        .arg(file_path)
        .arg("-o")
        .arg(&executable_name)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Compilation failed:");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                return;
            }

            // Run the compiled executable
            let run_result = Command::new(Path::new(&executable_name)).status();
            match run_result {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("Program execution failed");
                    }
                }
                Err(e) => eprintln!("Failed to execute program: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to compile: {}", e),
    }
}

fn print_help() {
    println!("Usage: {} [OPTIONS] <rust_file>", env::args().next().unwrap());
    println!("\nOptions:");
    println!("  -h, --help    Show this help message and list available Rust files");
    println!("\nExample:");
    println!("  {} example.rs", env::args().next().unwrap());
}
