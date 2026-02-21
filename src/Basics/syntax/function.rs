/*
fn => defines a function
() => parameters go inside
:Type => parameter type
-> => return type
no semicolon => returns value
pub => make function public
main => this name is entry point
 */

//Basic fn syntax
/*
fn function_name() {
    println!("Hello Rust");
}
*/

// 1️⃣ function with parameters
fn greet(name: &str) {
    println!("Hello, {}", name);
}

pub fn function_examples() {
    greet("rust");
    println!("{}", add(2, 3));
}
// name => parameter name
// : &str => parameter type

//2️⃣ Function With Return Value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//3️⃣ Using return Keyword (Optional)
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

//4️⃣ Function Without Return Value
fn say_hello() {
    println!("Hello!");
}

//5️⃣ Function With Multiple Parameters

fn print_info(name: &str, age: u8) {
    println!("Name: {}, Age: {}", name, age);
}

//6️⃣ Functions Returning Boolean
fn is_even(number: i32) -> bool {
    number % 2 == 0
}

//7️⃣ Public Functions (Modules)
pub fn my_function() {
    println!("This is public");
}