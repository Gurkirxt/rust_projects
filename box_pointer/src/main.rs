fn main() {
    let number = Box::new(5);

    println!("The number is: {}", number);

    let another_number = number;

    // println!("Number: {}", number);
    println!("Another number is: {}", another_number);
}

