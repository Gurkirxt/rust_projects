use std::env;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <temperature_value> <target_unit (C or F)>", args[0]);
        std::process::exit(1);
    }

    let value: f64 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: The temperature value must be a valid number.");
            std::process::exit(1);
        }
    };

    let unit = args[2].to_uppercase();

    match unit.as_str() {
        "F" => {
            let result = celsius_to_fahrenheit(value);
            println!("{value}°C is equal to {result:.2}°F");
        }
        "C" => {
            let result = fahrenheit_to_celsius(value);
            println!("{value}°F is equal to {result:.2}°C");
        }
        _ => {
            eprintln!("Error: The target unit must be either 'C' (Celsius) or 'F' (Fahrenheit).");
            std::process::exit(1);
        }
    }
}
