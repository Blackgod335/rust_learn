// Static variable (lives for entire program)
static MY_AGE: i32 = 25;

// Constant (must have type, always immutable)
const MIN_AGE: i32 = 18;

pub fn variables() {
    println!("--- Rust Variables Examples ---");

    // 1️⃣ Declaring Variables (Immutable by default)
    let x = 5;
    let name = "Karuppu";
    println!("x = {}", x);
    println!("name = {}", name);

    // 2️⃣ Mutable Variables
    let mut number = 5;
    number = 10;
    println!("number (mutable) = {}", number);

    // 3️⃣ Using Constants & Static
    println!("Minimum age = {}", MIN_AGE);
    println!("My age (static) = {}", MY_AGE);

    // 4️⃣ Shadowing (re-declaring same variable name)
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y after shadowing = {}", y);

    // 5️⃣ Type Annotations
    let a: i32 = 42;       // explicit type
    let big_number: i64 = 394_848;
    println!("a = {}", a);
    println!("big_number = {}", big_number);

    // 6️⃣ Variable Scope
    {
        let inside_block = 56;
        println!("inside_block = {}", inside_block);
    }
    // inside_block cannot be used here

    // 7️⃣ Destructuring
    let (num1, num2) = (1, 2);
    println!("num1 = {}, num2 = {}", num1, num2);
}