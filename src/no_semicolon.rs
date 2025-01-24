macro_rules! insert {
    ($($s:stmt)*) => {
        $($s);*
    }
}

fn main() {
    insert! {
        let mut x = 5
        x += 1
        println!("{}", x)
    };
}