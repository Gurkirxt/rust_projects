fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![7, 3, 4, 6, 8, 2, 1];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['t', 'y', 'u', 'i', 'a', 'd', 'g', 'b'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
