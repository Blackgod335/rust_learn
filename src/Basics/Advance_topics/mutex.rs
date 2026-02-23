/*
1. Why Mutex?

In Rust, normal variables are not thread-safe.
If two threads try to modify the same data simultaneously,
 you get a data race, which Rust prevents at compile time.

A Mutex<T> allows you to:

Share data between threads.

Ensure exclusive access when modifying the data.
*/

use std::sync::Mutex;

fn mutex_example() {
    let counter = Mutex::new(0); // Mutex protects the integer

    {
        let mut num = counter.lock().unwrap(); // Lock the mutex
        *num += 1; // Modify the value safely
    } // Mutex is automatically unlocked here

    println!("Counter: {:?}", counter);
}

/*
Explanation:

Mutex::new(0) – wraps a value (0) in a mutex.

counter.lock() – acquires the lock. Only one thread can hold the lock at a time.

*num += 1 – modifies the inner value safely.

When the num variable goes out of scope, the mutex is automatically unlocked.
*/

//Mutex Across Threads

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {:?}", counter.lock().unwrap()); // 5
}

/*
Arc allows multiple threads to own the same mutex.

lock() ensures only one thread modifies the data at a time.
*/