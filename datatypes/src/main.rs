fn main() {
    let int_var: i32 = 42;
    println!("Integer: {}", int_var);

    let float_var: f64 = 3.1415;
    println!("Float: {}", float_var);

    let bool_var: bool = true;
    println!("Boolean: {}", bool_var);

    let char_var: char = 'A';
    println!("Character: {}", char_var);

    let string_var: &str = "Hello, Rust!";
    println!("String: {}", string_var);

    let tuple_var: (i32, f64, bool) = (10, 5.5, false);
    println!("Tuple: {:?}", tuple_var);

    let array_var: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array_var);
}

