/*
1. What Arc Does

Allows shared ownership of a value across threads.

Uses atomic operations to keep track of the reference count safely in a multi-threaded environment.

When the last owner goes out of scope, the value is automatically dropped.

Key difference from Rc:

Rc<T> → single-threaded only

Arc<T> → multi-threaded safe
*/


use std::sync::Arc;
use std::thread;

fn arc_example() {
    let data = Arc::new(5);

    let mut handles = vec![];

    for _ in 0..3 {
        let data_clone = Arc::clone(&data); // increment reference count
        let handle = thread::spawn(move || {
            println!("Value: {}", data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/*
Arc::new(value) → creates a reference-counted pointer.

Arc::clone(&data) → increments the reference count safely across threads.

The value is dropped after all threads finish and the last Arc is gone.
*/