fn main() {
    let name = String::from("tris");
    let capitalized_name = capitalize(name.clone()); // Use clone to avoid moving

    println!("{}", capitalized_name); // OK
    println!("{}", name); // OK, because we used clone
    {
        let name = "tris";
        let capitalized_name = capitalize(name.to_string());

        println!("{}", capitalized_name); // OK
        println!("{}", name); // error: `name` moved
    }
}

fn capitalize(s: String) -> String {
    let mut capitalized = s.chars().collect::<Vec<_>>();
    if let Some(first_char) = capitalized.get_mut(0) {
        *first_char = first_char.to_ascii_uppercase();
    }
    capitalized.into_iter().collect()
}
