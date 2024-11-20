fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.iter();

    while let Some(number) = iter.next() {
        println!("{}", number);
    }

    for number in numbers.iter() {
        println!("{}", number);
    }

}

