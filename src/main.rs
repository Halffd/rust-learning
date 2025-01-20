fn abort() -> ! {
    panic!("This function will never return!");
}
fn check_value(x: i32) -> i32 {
    if x < 0 {
        abort(); // This will never return.
    }
    x
}
fn main() {
    println!("{:x}", 1692134);
    println!("{:o}", check_value(5));
    check_value(-20);
    // std::procechess::exit(1);
    //    abort();
}
