enum Vehicle {
    Car { make: String, year: u32 },
    Bike(String),
    Bus(u32, u32),
}

fn main() {
    let vehicles = vec![
        Vehicle::Car {
            make: "Toyota".to_string(),
            year: 2020,
        },
        Vehicle::Bike("Yamaha".to_string()),
        Vehicle::Bus(30, 50),
    ];

    for vehicle in vehicles {
        match vehicle {
            Vehicle::Car { make, year } if year > 2015 => {
                println!("Modern Car: {} (year: {})", make, year);
            }
            Vehicle::Bike(ref brand) => {
                println!("Bike brand: {}", brand);
            }
            Vehicle::Bus(seats, capacity) if capacity > 40 => {
                println!("Large bus with {} seats and capacity {}", seats, capacity);
            }
            _ => println!("Some other type of vehicle."),
        }
    }

    let number = 7;
    match number {
        1 | 2 => println!("The number is either 1 or 2"),
        3..=10 => println!("The number is between 3 and 10"),
        _ => println!("The number is out of range"),
    }

    let optional_value = Some(42);
    if let Some(val) = optional_value {
        println!("Value inside `Some`: {}", val);
    } else {
        println!("No value");
    }

    let mut stack = vec![1, 2, 3];
    println!("Popping values from the stack:");
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let (a, b) = (10, 20);
    println!("a = {}, b = {}", a, b);

    let nested_enum = Vehicle::Car {
        make: "Ford".to_string(),
        year: 2012,
    };

    match nested_enum {
        Vehicle::Car { make, year } => match year {
            y if y < 2010 => println!("Old {} car (year: {})", make, year),
            y if y < 2020 => println!("Relatively new {} car (year: {})", make, year),
            _ => println!("Very new {} car!", make),
        },
        _ => println!("Not a car!"),
    }
}

