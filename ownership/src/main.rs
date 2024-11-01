fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s2);

    let s3 = String::from("Rust");
    let length = calculate_length(&s3);
    println!("The length of '{}' is {}", s3, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
