fn ifState() {
    /* The if statement is the most basic conditional in Rust.
    It executes a block of code only if the condition is true. */
    let number: i8 = 10;
    if number > 5 {
        println!("Number is greater than 5");
    }
}

fn elseState() {
    let number = 3;
    if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5 or less");
    }
}

fn elseIfState() {
    let number = 7;

    if number > 10 {
        println!("Greater than 10");
    } else if number > 5 {
        println!("Greater than 5 but not more than 10");
    } else {
        println!("5 or less");
    }
}

fn matchState() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other number"), // default case
    }
}

fn ifLetState() {
    //Simpler syntax for match when you care about only one pattern.
    let some_value = Some(5);

    if let Some(x) = some_value {
        println!("Value is {}", x);
    }
}

fn whileLetState() {
    let mut numbers = vec![1, 2, 3];

    while let Some(x) = numbers.pop() {
        println!("Popped {}", x);
    }
}

fn ternaryLikeState(){
    let number = 10;
    let result = if number > 5 { "Big" } else { "Small" };

    println!("Number is {}", result);
}

fn combiningState(){
     let x = 5;
    let y = 10;

    if x > 0 && y > 5 {
        println!("Both conditions are true");
    }

    if x < 0 || y > 5 {
        println!("At least one condition is true");
    }
}

fn logicalState(){
    let is_raining = false;

    if !is_raining {
        println!("Go outside!");
    }
}

fn rangeState(){
    let number = 7;

    if (1..=10).contains(&number) {
        println!("Number is between 1 and 10");
    }
}

pub fn condtions() {
    // 1. if

    // 2. else
    // 3. else if
    // 4. match
    // 5. if let
    // 6. while let
    // 7. Ternary-like Conditional Operator
    // 8. combining conditions
    // 9. logical conditions
    // 10. Range conditions
}
