/*

| Concept     | Meaning                       |   |    |
| ----------- | ----------------------------- | - | -- |
| `if`        | Run code if condition is true |   |    |
| `else`      | Run when condition is false   |   |    |
| `else if`   | Check another condition       |   |    |
| `match`     | Compare exact values          |   |    |
| `if let`    | Match one pattern (like Some) |   |    |
| `while let` | Loop while pattern matches    |   |    |
| `&&`        | AND                           |   |    |
| `           |                               | ` | OR |
| `!`         | NOT                           |   |    |
| `..=`       | Range inclusive               |   |    |

*/
pub fn conditions() {
    simple_if();
    simple_else();
    simple_else_if();
    simple_match();
    simple_if_let();
    simple_while_let();
    ternary_like();
    combine_conditions();
    logical_condition();
    range_condition();
}

//Runs code only if condition is true
fn simple_if() {
    let number = 10;

    if number > 5 {
        println!("Number is greater than 5");
    }
}

// Runs one block if true, another if false.
fn simple_else() {
    let number = 3;

    if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5 or less");
    }
}

//Checks multiple conditions
fn simple_else_if() {
    let number = 7;

    if number > 10 {
        println!("Greater than 10");
    } else if number > 5 {
        println!("Between 6 and 10");
    } else {
        println!("5 or less");
    }
}

//when checking specific values.
fn simple_match() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other number"), // default case
    }
}

//if let (Simpler match for one pattern)
// Used mostly with Option or Result.

fn simple_if_let() {
    let value = Some(5);

    if let Some(x) = value {
        println!("Value is {}", x);
    }
}

//Loop while pattern matches
fn simple_while_let() {
    let mut numbers = vec![1, 2, 3];

    while let Some(x) = numbers.pop() {
        println!("Popped {}", x);
    }
}

//Ternary-like Expression
fn ternary_like() {
    let number = 10;

    let result = if number > 5 { "Big" } else { "Small" };

    println!("Number is {}", result);
}

//Combining Conditions
fn combine_conditions() {
    let x = 5;
    let y = 10;

    if x > 0 && y > 5 {
        println!("Both are true");
    }

    if x < 0 || y > 5 {
        println!("At least one is true");
    }
}

//Logical NOT

fn logical_condition() {
    let is_raining = false;

    if !is_raining {
        println!("Go outside!");
    }
}

//Range Check
fn range_condition() {
    let number = 7;

    if (1..=10).contains(&number) {
        println!("Number is between 1 and 10");
    }
}