enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MyOption<U> {
        match self {
            MyOption::Some(x) => MyOption::Some(f(x)),
            MyOption::None => MyOption::None,
        }
    }

    fn unwrap_or<U>(self, default: U) -> U
    where
        T: Into<U>,
    {
        match self {
            MyOption::Some(x) => x.into(),
            MyOption::None => default,
        }
    }
}

fn main() {
    let possibly_a_number = MyOption::Some(1);
    let result = possibly_a_number.map(|n| n + 1).unwrap_or(0);
    println!("{}", result); // Output: 2
}
