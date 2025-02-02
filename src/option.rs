fn main() {
    let possibly_a_number = Some(1);
    let result = possibly_a_number.map(|n| n + 1).unwrap_or(0);
    println!("{}", result); // Output: 2
}
