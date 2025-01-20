fn abort() -> ! {
    panic!("This function will never return!");
}
fn main() {
    println!("Hello, world!");
    abort();
}
