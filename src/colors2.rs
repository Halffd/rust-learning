use colored::Colorize;
use std::f32::consts::PI;

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h / 60.0).floor() as i32 % 6 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

fn main() {
    let input = str::replace("Hello, T!", "T", "E");
    let length = input.chars().count() as f32;

    for (i, c) in input.chars().enumerate() {
        let hue = (i as f32 / length) * 360.0;
        let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.75);

        print!("{}", c.to_string().truecolor(r, g, b));
    }

    println!(); // Add final newline
}
