//🦀 What is an Enum?
//An enum (short for enumeration) is a type that can be one of several variants.

/*
🔹 Advantages of Enums

Multiple Variants – A value can be one of many types.

Pattern Matching Friendly – Works perfectly with match and if let.

Safety – Forces you to handle all variants (or use _).
 */

//Basic enums syntax

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn enum_example() {
    let player_direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Moving Up!"),
        Direction::Down => println!("Moving Down!"),
        Direction::Left => println!("Moving Left!"),
        Direction::Right => println!("Moving Right!"),
    }
}
