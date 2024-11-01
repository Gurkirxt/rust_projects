use std::io;

fn main() {
    loop {
        println!(
            "Enter operation to perform
                  1. Sum
                  2. Subtract
                  3. Multiply
                  4. Divide
                  5. exit"
        );
        let mut inputopt = String::new();
        io::stdin()
            .read_line(&mut inputopt)
            .expect("Failed to read");
        let opt: i8 = inputopt.trim().parse().expect("Failed to get number");
        let mut input = String::new();
        if opt == 5 {
            break;
        }
        println!("Enter First Number : ");
        io::stdin().read_line(&mut input).expect("Failed to read");
        let num1: f64 = input.trim().parse().expect("Failed to get number");
        let mut input2 = String::new();
        println!("Enter Second number : ");
        io::stdin().read_line(&mut input2).expect("Failed to read");
        let num2: f64 = input2.trim().parse().expect("Failed to get number");
        match opt {
            1 => {
                println!("{num1} + {num2} = {}", (num1 + num2))
            }
            2 => {
                println!("{num1} - {num2} = {}", (num1 - num2))
            }
            3 => {
                println!("{num1} X {num2} = {}", (num1 * num2))
            }
            4 => {
                println!("{num1} / {num2} = {}", (num1 / num2))
            }
            5 => break,
            _ => println!("Invalid input"),
        }
    }
}
