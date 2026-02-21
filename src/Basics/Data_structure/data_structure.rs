// 1. Integer
/*
# unsigned numbers

Type	Minimum	      Maximum
u8	        0	        2^8-1
u16     	0	        2^16-1
u32     	0       	2^32-1
u64     	0	        2^64-1
u128    	0       	2^128-1

# signed numbers

Type	    Minimum	       Maximum
i8	        -(2^7)	        2^7-1
i16     	-(2^15)      	2^15-1
i32     	-(2^31)	        2^31-1
i64     	-(2^63)	        2^63-1
i128    	-(2^127)     	2^127-1


 */

//  2. Floats
/*
Rust supports two types of floating-point numbers: f32 and f64. These are 32-bit and 64-bit in size, respectively.
 */

// 3. bool
/*
Rust's bool primitive type represents truth values with two possible states: true or false.

AND => &&
OR  => ||
NOT => !
 */

//4. char
/*
 Rust's char type represents a Unicode Scalar Value,
  supporting far more than ASCII including emojis,
   accented letters, and various scripts.
    Each char occupies 4 bytes (32 bits) in memory and
     is defined using single quotes.

Example: let letter: char = 'z'; or let emoji: char = '🦀';
  */

//5. string
/*
String is heap-allocated, growable, and mutable.

Supports UTF-8, so you can store any text, including emojis.

For read-only string slices, use &str.
 */
// # create string
fn string_create() {
    //Creating Strings
    let s1 = String::from("Hello"); // Using String::from
    let s2 = "World".to_string(); // Using to_string() method

    println!("{} {}", s1, s2); // Output: Hello World

    //Appending to Strings
    //Append a string slice: push_str()
    let mut s = String::from("Hello");
    s.push_str(", Rust!");
    println!("{}", s); // Hello, Rust!

    //Append a single character: push()
    let mut s = String::from("Hi");
    s.push('!');
    println!("{}", s); // Hi!

    //Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    println!("{}", s3); // Hello, World!

    //Using format! macro (does not take ownership):

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3); // Hello Rust

    //others
    let mut s = String::from("Rust");

    // Length in bytes
    println!("Length: {}", s.len());

    // Capacity in bytes
    println!("Capacity: {}", s.capacity());

    // Check if empty
    println!("Is empty? {}", s.is_empty());

    // Remove last character
    s.pop();
    println!("{}", s); // Rus
}

// 6. Tuples

//Tuples are fixed-size collections that can hold elements of different types.

fn tuples_examples() {
    let person: (&str, u8, bool) = ("Alice", 30, true);
    println!(
        "Name: {}, Age: {}, Active: {}",
        person.0, person.1, person.2
    );

    let t = ("Rust", 2026, 3.14, true);
    println!("Language: {}, Year: {}", t.0, t.1);

    //destructing
    let person = ("Bob", 25, false);
    let (name, age, active) = person; // Destructuring
    println!("Name: {}, Age: {}, Active: {}", name, age, active);

    //functions

    fn min_max(numbers: &[i32]) -> (i32, i32) {
        let min = *numbers.iter().min().unwrap();
        let max = *numbers.iter().max().unwrap();
        (min, max)
    }

    fn number() {
        let numbers = [3, 7, 1, 9, 4];
        let (min, max) = min_max(&numbers);
        println!("Min: {}, Max: {}", min, max); // Min: 1, Max: 9
    }

    //Nested tuples

    let nested = ((1, 2), ("Rust", true));
    println!("First: {}", (nested.0).0); // Access 1
    println!("Second: {}", (nested.1).0); // Access "Rust"
}

// 7. Arrays

/*Arrays are fixed-size collections of elements of the same type stored consecutively in memory.
 Size must be known at compile time and cannot change. Syntax:let arr: [type; size] = [elements];
 ex : et nums: [i32; 3] = [1, 2, 3];
 Access elements with zero-based indexing: arr[0].
*/

// 8. Vector

/*
Vec<T> is Rust's growable, heap-allocated array that stores elements of the same
type contiguously. Unlike arrays, vectors can resize at runtime.
Key methods include push() to add elements, pop() to remove the last element,
and len() for size.
 Example: let mut v = vec![1, 2, 3];

 */

//creating vector

fn vector_example() {
    // Empty vector, type annotated
    let mut v: Vec<i32> = Vec::new();

    // Using vec! macro
    let mut v2 = vec![1, 2, 3, 4, 5];

    println!("{:?}", v2); // Output: [1, 2, 3, 4, 5]
}
//Use Vec::new() for an empty vector.

// Use vec![...] to initialize with values.


// 9. Hash map(key-value pairs)

/*
HashMap::new() =>  creates an empty map.
HashMap::new() =>  creates an empty map.
.get("key")  =>    access the value
.insert(key, value)     =>     insert new data
.remove(key)        =>      delete the key and value
.entry(key).or_insert(value)   => if key exists do nothing or if key doesnot exists insert value
 */

use std::collections::HashMap;

fn hashMap_example() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    println!("{:?}", scores);
}


// 10. Hashset(no duplicates)

use std::collections::HashSet;

fn hashset_examples() {
    let mut set = HashSet::new();

    set.insert(10);
    set.insert(20);
    set.insert(10); // duplicate (ignored)

    println!("{:?}", set);
}

// duplicate are automaticly ignored
// order is not guaranteed


// 11. Linked List(doubly linked list collection)

/*
.push_back()    =>      add to end
.push_front()   =>      add to beginning
.pop_front()    =>      remove the first element
.pop_back()     =>      remove the last element

*/

use std::collections::LinkedList;

fn linked_list_examples() {
    let mut list = LinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_front(5);

    println!("{:?}", list); // [5, 10, 20]
}


// 12. Stack(LIFO – Last In, First Out)
// .push() and .pop() and .last()
fn stack_example() {
    let mut stack = Vec::new();

    // Push elements
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Stack: {:?}", stack);

    // Pop elements (LIFO)
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

// 13.Queue(FIFO - First in first out)

use std::collections::VecDeque;

fn queue_example() {
    let mut queue = VecDeque::new();

    // Enqueue (add to back)
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);

    println!("Queue: {:?}", queue);

    // Dequeue (remove from front)
    while let Some(front) = queue.pop_front() {
        println!("Removed: {}", front);
    }
}


// 14. BinaryHeap (priority queue)

use std::collections::BinaryHeap;

fn binaryHeap_example() {
    let mut heap = BinaryHeap::new();

    heap.push(10);
    heap.push(30);
    heap.push(20);

    println!("{:?}", heap);
}