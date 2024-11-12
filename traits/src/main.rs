trait Describe {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u32,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}

struct Animal {
    species: String,
}

impl Describe for Animal {
    fn describe(&self) -> String {
        format!("This is a {}.", self.species)
    }
}

fn print_description<T: Describe>(item: T) {
    println!("{}", item.describe());
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let animal = Animal {
        species: String::from("Dog"),
    };

    print_description(person);
    print_description(animal);
}
