//Functions defined within an impl block can use self to access instance data.

struct Circle {
    radius: f64,
}

impl Circle {
    // Method to calculate area
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // Associated function
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

fn main() {
    let circle = Circle::new(5.0);
    println!("Area: {}", circle.area());
}
