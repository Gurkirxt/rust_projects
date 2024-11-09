fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    let hello = &s[0..5]; 

    for c in s.chars() {
        println!("{}", c);
    }

    for b in s.bytes() {
        println!("{}", b);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}",s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
    println!("{}", hello);
    println!("{}", s3);
    println!("{}", s4);
}

