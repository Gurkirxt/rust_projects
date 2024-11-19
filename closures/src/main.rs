fn main() {
    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("Count is now: {}", count);
    };

    increment();
    increment();

    println!("Final count: {}", count);
}

