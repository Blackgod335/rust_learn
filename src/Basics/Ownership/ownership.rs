/*
1. What is Ownership?
    In Rust, ownership is a set of rules that governs how memory is managed.
It’s Rust’s way of ensuring memory safety without a garbage collector.
*/

fn ownership_example() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // Ownership moves from s1 to s2

    // println!("{}", s1); // ❌ Error: s1 is no longer valid
    println!("{}", s2); // ✅ Works
}

//s1 originally owns the string.

// # Borrowing

//Sometimes you want to let another part of your code use a value without taking ownership. This is called borrowing.

// & => borrow the value without taking ownership

fn borrow_example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1
    println!("Length of '{}' is {}", s1, len); // ✅ s1 still valid
}

fn calculate_length(s: &String) -> usize {
    s.len() // Use the reference
}

// mutable borrow reference
fn mutable_borrow() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // ✅ "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/*
     Why Ownership Matters

Memory safety – Rust prevents use-after-free and double-free bugs at compile time.

No garbage collector – Rust can automatically clean up memory when variables go out of scope.

Efficient performance – Ownership enables zero-cost abstractions.
*/
