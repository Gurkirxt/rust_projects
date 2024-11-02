use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Unable to read the file");
    let mut iter = contents.split_ascii_whitespace();
    let mut count: u128 = 0;
    while iter.next() != None {
        count = count + 1;
    }
    println!("Number of words = {}", count);
    let mut count_char: usize = 0;
    let characters: Vec<char> = contents.chars().collect();
    for c in characters {
        if c.is_ascii() {
            count_char = count_char + 1;
        }
    }
    println!("Number of characters = {}", count_char);
}
