//1. Option<T>

//Option<T> is used when a value may or may not exist.

enum Option<T> {
    Some(T),
    None,
}

//Some(value) → the value exists.
//None → the value is absent.


fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 6];
    match find_even(&numbers) {
        Some(n) => println!("Found even number: {}", n),
        None => println!("No even number found"),
    }
}


//2. Result<T, E>

//Result<T, E> is used for operations that can succeed or fail.

enum Result<T, E> {
    Ok(T),
    Err(E),
}

/*
Ok(value) → operation succeeded, contains the result.

Err(error) → operation failed, contains the error.
*/

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}