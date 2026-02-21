// A struct is defined using the struct keyword and can contain various types of fields.

struct Person {
    name: String,
    age: u32,
}

pub fn structs() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
}
