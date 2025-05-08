fn subcadeia(c: &str, p: usize, n: usize, depth: usize) -> String {
    println!(
        "{:indent$}CALL -> c: '{}', p: {}, n: {}",
        "", c, p, n, indent = depth * 2
    );

    if n == 0 {
        println!("{:indent$}RETURN -> '' (base case)", "", indent = depth * 2);
        return String::from("");
    } else {
        if p > 1 {
            let result = subcadeia(&c[1..], p - 1, n, depth + 1);
            println!(
                "{:indent$}RETURN -> '{}' (after skipping)",
                "", result, indent = depth * 2
            );
            return result;
        } else {
            let first_char = c.chars().next().unwrap_or('\0');
            let mut result = String::new();
            result.push(first_char);
            let remainder = subcadeia(&c[1..], 1, n - 1, depth + 1);
            result.push_str(&remainder);

            println!(
                "{:indent$}RETURN -> '{}' (after taking)",
                "", result, indent = depth * 2
            );
            return result;
        }
    }
}

fn subcadeia2(c: &str, p: usize, n: usize) -> String {
    c.chars().skip(p - 1).take(n).collect()
}

fn main() {
    let strings = [
        "Hello, World!",
        "Rust is awesome",
        "Learning programming",
        "Enjoy coding every day!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
    ];

    for (i, s) in strings.iter().enumerate() {
        let p = 2; // Starting position
        let n = 7; // Number of characters to extract
        let result = subcadeia(s, p, n, 0);
        println!("Substring from string {}: {}", i + 1, result);
    }
}
fn ultra_smart_subcadeia(c: &str, p: usize, n: usize) -> &str {
    &c.as_bytes()[(p-1)..(p-1+n)]
}
