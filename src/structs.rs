struct Number {
    odd: bool,
    value: i32,
}

struct Pair<T> {
    a: T,
    b: T,
}

impl Number {
    fn is_positive(&self) -> bool {
        self.value > 0
    }
}

// Generic function examples
fn foobar_single<T>(arg: T) {
    println!("Called foobar_single with some value");
}

fn foobar_double<L, R>(left: L, right: R) {
    println!("Called foobar_double with two different types");
}

fn print_number(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Other number"),
    }
}

fn main() {
    // Generic struct examples
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };

    // Generic function calls
    foobar_single(42);
    foobar_single("hello");
    foobar_double(3.14, false);
    foobar_double('x', vec![1, 2, 3]);

    // Previous number examples
    let mut n = Number { odd: true, value: 17 };
    n.odd = false;

    let minus_two = Number { odd: false, value: -2 };
    println!("Is positive? {}", minus_two.is_positive());

    let x = Number { odd: false, value: 2 };
    let y = Number { value: 3, odd: true };
    
    print_number(x);
    print_number(y);

    // String length examples
    let length1 = "amos".len();
    let length2 = str::len("amos");
    println!("Length using method syntax: {}", length1);
    println!("Length using function syntax: {}", length2);
}