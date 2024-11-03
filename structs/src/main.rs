struct Rectangle{
    side_a: i64,
    side_b: i64,
}

impl Rectangle{
    fn area(&self)->i64{
        self.side_a * self.side_b
    }
    fn square(size:i64)->Rectangle{
        Rectangle{
            side_a : size,
            side_b : size,
        }
    }
}

fn main() {
    let rect = Rectangle{
        side_a : 10,
        side_b : 20,
    };
    println!("Area of rect = {}",rect.area());
    let sq = Rectangle :: square(30);
    println!("Area of square = {}",sq.area());
}
