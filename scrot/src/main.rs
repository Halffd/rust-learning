use clap::{Parser, Subcommand};
use scrap::{Capturer, Display};
use rdev::{EventType, listen, Key};
use image::{ImageBuffer, Rgba};
use std::error::Error;
use std::sync::{Arc, Mutex};

// Define the CLI structure using clap
#[derive(Parser)]
#[command(name = "screenshot")]
#[command(about = "A CLI tool to capture screenshots of monitors or selected areas")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Define subcommands for full screen capture and area selection
#[derive(Subcommand)]
enum Commands {
    /// Capture full screen(s)
    Full {
        /// Capture all monitors
        #[arg(long)]
        all: bool,
        /// Capture specific monitor by index (e.g., 0, 1, 2)
        #[arg(long)]
        monitor: Option<usize>,
        /// Output file or directory (default: screenshot_monitor_<index>.png or screenshot.png)
        #[arg(long)]
        output: Option<String>,
    },
    /// Select an area to capture
    Area {
        /// Output file (default: screenshot_area.png)
        #[arg(long)]
        output: Option<String>,
    },
}

// State machine for area selection
enum SelectionState {
    Idle,
    Selecting { start_x: f64, start_y: f64 },
    Selected { start_x: f64, start_y: f64, end_x: f64, end_y: f64 },
    Cancelled,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Full { all, monitor, output } => {
            let displays = Display::all()?;
            if all {
                // Capture all monitors
                for (i, display) in displays.into_iter().enumerate() {
                    let output_path = format!("screenshot_monitor_{}.png", i);
                    capture_display(display, &output_path)?;
                    println!("Captured monitor {} to {}", i, output_path);
                }
            } else if let Some(index) = monitor {
                // Capture specific monitor
                let displays = Display::all()?;
                if index < displays.len() {
                    let output_path = output.unwrap_or_else(|| format!("screenshot_monitor_{}.png", index));
                    capture_display(displays.into_iter().nth(index).unwrap(), &output_path)?;
                    println!("Captured monitor {} to {}", index, output_path);
                } else {
                    println!("Error: Monitor index {} is invalid. Available monitors: {}", index, displays.len());
                }
            } else {
                // Default to primary monitor
                let primary = Display::primary()?;
                let output_path = output.unwrap_or("screenshot.png".to_string());
                capture_display(primary, &output_path)?;
                println!("Captured primary monitor to {}", output_path);
            }
        }
        Commands::Area { output } => {
            // Capture selected area
            match select_area() {
                Ok((display, local_left, local_top, region_width, region_height)) => {
                    let output_path = output.unwrap_or("screenshot_area.png".to_string());
                    capture_region(display, local_left, local_top, region_width, region_height, &output_path)?;
                    println!("Captured selected area to {}", output_path);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
    Ok(())
}

// Capture the entire display and save it as a PNG
fn capture_display(display: Display, output_path: &str) -> Result<(), Box<dyn Error>> {
    let width = display.width() as u32;
    let height = display.height() as u32;
    let mut capturer = Capturer::new(display)?;
    let frame = capturer.frame()?;
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    // Convert BGRA (scrap format) to RGBA (image crate format)
    for pixel in frame.chunks(4) {
        let b = pixel[0];
        let g = pixel[1];
        let r = pixel[2];
        let a = pixel[3];
        pixels.push(r);
        pixels.push(g);
        pixels.push(b);
        pixels.push(a);
    }

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, pixels)
        .ok_or("Failed to create image buffer")?;
    img.save(output_path)?;
    Ok(())
}

// Handle area selection using mouse events
fn select_area() -> Result<(Display, i32, i32, i32, i32), Box<dyn Error>> {
    println!("Click and drag to select an area. Press Esc to cancel.");
    let displays = Display::all()?;
    let state = Arc::new(Mutex::new(SelectionState::Idle));
    let state_clone = state.clone();

    // Create a channel to handle the listen result
    let (tx, rx) = std::sync::mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    listen(move |event: rdev::Event| -> bool {
        let mut state = state_clone.lock().unwrap();
        match *state {
            SelectionState::Idle => {
                if let EventType::ButtonPress(_) = event.event_type {
                    *state = SelectionState::Selecting {
                        start_x: event.position.x,
                        start_y: event.position.y,
                    };
                } else if let EventType::KeyPress(key) = event.event_type {
                    if key == Key::Escape {
                        *state = SelectionState::Cancelled;
                    }
                }
            }
            SelectionState::Selecting { start_x, start_y } => {
                if let EventType::ButtonRelease(_) = event.event_type {
                    *state = SelectionState::Selected {
                        start_x,
                        start_y,
                        end_x: event.position.x,
                        end_y: event.position.y,
                    };
                } else if let EventType::KeyPress(key) = event.event_type {
                    if key == Key::Escape {
                        *state = SelectionState::Cancelled;
                    }
                }
            }
            _ => {}
        }

        let should_stop = matches!(*state, SelectionState::Selected { .. } | SelectionState::Cancelled);
        if should_stop {
            if let Ok(tx) = tx.lock() {
                let _ = tx.send(());
            }
            false
        } else {
            true
        }
    }).map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // Wait for selection to complete
    let _ = rx.recv();

    let state = state.lock().unwrap();
    match &*state {
        SelectionState::Selected { start_x, start_y, end_x, end_y } => {
            // Calculate the selected region in global coordinates
            let left = start_x.min(*end_x);
            let top = start_y.min(*end_y);
            let width = (start_x - end_x).abs();
            let height = (start_y - end_y).abs();

            // Find the display containing the starting point
            let display = displays
                .into_iter()
                .find(|d| {
                    let x = d.width() as f64;
                    let y = d.height() as f64;
                    *start_x >= 0.0 && *start_x < x &&
                    *start_y >= 0.0 && *start_y < y
                })
                .ok_or("No display found for the selected area")?;

            // Convert coordinates to integers and clamp to display bounds
            let local_left = left.round() as i32;
            let local_top = top.round() as i32;
            let region_width = width.round() as i32;
            let region_height = height.round() as i32;

            if region_width <= 0 || region_height <= 0 {
                return Err("Selected area is invalid.".into());
            }

            Ok((display, local_left, local_top, region_width, region_height))
        }
        SelectionState::Cancelled => Err("Selection cancelled by user.".into()),
        _ => unreachable!(),
    }
}

// Capture a specific region from a display
fn capture_region(
    display: Display,
    local_left: i32,
    local_top: i32,
    region_width: i32,
    region_height: i32,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let width = display.width() as usize;
    let mut capturer = Capturer::new(display)?;
    let frame = capturer.frame()?;
    let stride = width * 4; // Bytes per row (BGRA)
    let local_left = local_left as usize;
    let local_top = local_top as usize;
    let region_width = region_width as usize;
    let region_height = region_height as usize;
    let mut region_pixels = Vec::with_capacity(region_width * region_height * 4);

    // Extract the selected region and convert BGRA to RGBA
    for y in local_top..(local_top + region_height) {
        let row_start = y * stride + local_left * 4;
        for x in 0..region_width {
            let idx = row_start + x * 4;
            let b = frame[idx];
            let g = frame[idx + 1];
            let r = frame[idx + 2];
            let a = frame[idx + 3];
            region_pixels.push(r);
            region_pixels.push(g);
            region_pixels.push(b);
            region_pixels.push(a);
        }
    }

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(region_width as u32, region_height as u32, region_pixels)
        .ok_or("Failed to create image buffer for region")?;
    img.save(output_path)?;
    Ok(())
}