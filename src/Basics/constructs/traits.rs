//1. What Is a Trait?
/*
In Rust, traits are a powerful way to define shared behavior that multiple types can implement.
They allow for polymorphism, enabling different types to provide their own implementations of the same behavior.
*/


// Define a trait
trait Speak {
    fn speak(&self);
}

// Implement the trait for a type
struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

pub fn traits() {
    let dog: Dog = Dog;
    let cat: Cat = Cat;

    dog.speak(); // Woof!
    cat.speak(); // Meow!
}