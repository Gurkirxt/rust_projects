fn main() {
    let mut numbers = Vec::new();

    for i in 1..=5 {
        numbers.push(i);
    }

    println!("Numbers: {:?}", numbers);

    let second = &numbers[1];
    println!("The second element is {}", second);

    match numbers.get(3) {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("There is no fourth element."),
    }

    numbers.push(6);
    numbers.push(7);

    for number in &mut numbers {
        *number *= 2;
    }

    println!("Doubled numbers: {:?}", numbers);

    let initial_vector = vec![100, 200, 300];
    println!("Initial vector: {:?}", initial_vector);

    let sum: i32 = initial_vector.iter().sum();
    println!("Sum of initial vector: {}", sum);
}

