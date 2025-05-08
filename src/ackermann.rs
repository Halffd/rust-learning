use std::time::Instant;

fn ackermann(m: u32, n: u32, depth: usize) -> u32 {
    let indent = "  ".repeat(depth);
    println!("{}ackermann({}, {}) [Call]", indent, m, n);

    let start = Instant::now();
    let result = if m == 0 {
        let res = n + 1;
        println!(
            "{}ackermann({}, {}) [Return {}] (Base case m=0)",
            indent, m, n, res
        );
        res
    } else if n == 0 {
        let res = ackermann(m - 1, 1, depth + 1);
        println!(
            "{}ackermann({}, {}) [Return {}] (Case n=0)",
            indent, m, n, res
        );
        res
    } else {
        let inner = ackermann(m, n - 1, depth + 1);
        let res = ackermann(m - 1, inner, depth + 1);
        println!(
            "{}ackermann({}, {}) [Return {}] (Recursive case)",
            indent, m, n, res
        );
        res
    };

    let elapsed = start.elapsed();
    println!("{}ackermann({}, {}) [Time {:?}]", indent, m, n, elapsed);
    result
}

fn timed_ackermann(m: u32, n: u32) -> u32 {
    println!("\nComputing ackermann({}, {})", m, n);
    let start = Instant::now();
    let result = ackermann(m, n, 0);
    let elapsed = start.elapsed();
    println!("\nFinal result: {}", result);
    println!("Total execution time: {:?}", elapsed);
    result
}

fn main() {
    timed_ackermann(200, 100);
}
