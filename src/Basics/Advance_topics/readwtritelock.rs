/*
1. What is an RwLock?

Multiple readers can access the data simultaneously.

Only one writer can access the data, and while writing, no readers are allowed.

Prevents data races in concurrent scenarios.
*/

use std::sync::RwLock;

fn rwlock_example() {
    let lock = RwLock::new(5);

    // Reading the value
    {
        let r = lock.read().unwrap();
        println!("Read value: {}", *r);
    } // read lock released here

    // Writing the value
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("Updated value: {}", *w);
    } // write lock released here
}

/*
lock.read() → acquires a shared read lock. Multiple threads can do this simultaneously.

lock.write() → acquires an exclusive write lock. Only one thread can write, and no reads are allowed during writing.
*/