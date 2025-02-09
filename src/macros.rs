macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let result = add!(1, 2); // this line will be replaced with `1 + 2`
    println!("The result is: {}", result);
}
