use std::mem;

#[derive(PartialEq, Eq)]
struct Male {
 worth: u32
}

#[derive(PartialEq, Eq)]
struct Female {
 worth: u32
}

fn main() {
 let first: Male = Male { worth: 100 };
 let second: Female = Female { worth: 100 };
 let new_first: Female = unsafe { mem::transmute(first) };
 println!("{}", new_first == second); // true
}