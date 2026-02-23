/*
1. What is a Channel?

A channel has two ends:

Sender (tx) – sends messages.

Receiver (rx) – receives messages.

Channels are thread-safe, so you don’t need a Mutex or RwLock to coordinate access.

They can be single-producer, single-consumer (SPSC) or multi-producer,
single-consumer (MPSC). Rust’s standard library provides MPSC channels.
*/


use std::sync::mpsc;
use std::thread;

fn cahnnel_example() {
    let (tx, rx) = mpsc::channel(); // Create a channel

    thread::spawn(move || {
        tx.send("Hello from thread").unwrap(); // Send a message
    });

    let received = rx.recv().unwrap(); // Receive the message
    println!("Received: {}", received);
}