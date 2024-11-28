macro_rules! factorial {
    ($n:expr) => {
        {
            let mut result = 1;
            let mut i = 1;
            while i <= $n {
                result *= i;
                i += 1;
            }
            result
        }
    };
}

fn main() {
    let num = 5;
    let fact = factorial!(num);
    println!("The factorial of {} is {}", num, fact);
}


