/*
Rust Loop Types

1. loop   -> Infinite loop (runs forever until break)
2. while  -> Runs while condition is true
3. for    -> Loops over a range or collection

Keywords:
- break    -> Stops the current loop
- continue -> Skips to the next iteration
*/

pub fn loops() {
    loop_example();
    while_example();
    for_example();
    loop_label_example();
    search_in_grid_example();
}

fn loop_example() {
    println!("--- loop example ---");

    let mut count = 0;

    loop {
        if count >= 5 {
            break; // stop the loop
        }

        println!("Count: {}", count);
        count += 1;
    }
}

fn while_example() {
    println!("--- while example ---");

    let mut number = 5;

    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }
}

fn for_example() {
    println!("--- for example ---");

    for i in 1..=5 {
        println!("Value: {}", i);
    }
}

//loop labels
//Loop labels are used when you have nested loops
fn loop_label_example() {
    println!("--- loop label example ---");

    // Example 1: Break outer loop
    'outer: loop {
        println!("Outer loop");

        loop {
            println!("Inner loop");
            break 'outer; // breaks outer loop
        }
    }

    println!("Finished first label example");

    // Example 2: Continue outer loop
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if j == 2 {
                continue 'outer; // skip to next i
            }

            println!("i = {}, j = {}", i, j);
        }
    }
}

fn search_in_grid_example() {
    println!("--- search grid example ---");

    let grid = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let target = 9;

    'search: for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == target {
                println!(
                    "Found {} at row {}, column {}",
                    target, row, col
                );

                break 'search; // stop searching completely
            }
        }
    }
}