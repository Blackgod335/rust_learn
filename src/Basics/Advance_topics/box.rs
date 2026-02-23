/*
1. What Box Does

Heap-allocates a value.

The variable that holds the Box<T> owns the data.

When the Box goes out of scope, the heap memory is automatically freed.

Useful for:

Storing large data without copying it on the stack.

Recursive data structures (like linked lists or trees).

Trait objects (dynamic dispatch).
*/

fn box_example() {
    let x = 5;          // stored on the stack
    let y = Box::new(10); // stored on the heap

    println!("x = {}, y = {}", x, y);
}

//  x is stored on the stack.
//  y is a Box pointing to a value on the heap.