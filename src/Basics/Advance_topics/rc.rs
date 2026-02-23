/*
1. What Rc Does

Keeps track of how many references (owners) exist for a value.

When the last owner goes out of scope, the value is automatically dropped.

Only works in single-threaded code. For multi-threaded shared ownership, you use Arc<T>.
*/

use std::rc::Rc;

fn rc_example() {
    let a = Rc::new(5); // create an Rc pointing to 5
    let b = Rc::clone(&a); // clone the Rc (increment reference count)

    println!("a = {}, b = {}", a, b);
    println!("Reference count = {}", Rc::strong_count(&a)); // 2
}

/*
Rc::new(value) → creates a reference-counted pointer.

Rc::clone(&a) → increments the reference count, giving another owner.

Rc::strong_count(&a) → returns how many owners exist.

 Why Use Rc?

Useful when multiple parts of your program need read-only access to the same data.

Prevents copying large data unnecessarily.

Commonly used in tree-like or graph data structures, e.g., multiple nodes sharing a child.
*/