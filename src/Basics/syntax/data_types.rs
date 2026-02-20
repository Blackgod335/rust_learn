use std::mem;

pub fn data_types() {
    println!("===== RUST DATA TYPES =====");

    // ---------------------------------
    // 1️⃣ INTEGER TYPES
    // ---------------------------------
    // 1 byte = 8 bits
    // Signed = positive + negative
    // Unsigned = only positive

    println!("\n-- Integer Types --");

    println!("i8   size: {} byte", mem::size_of::<i8>());
    println!("u8   size: {} byte", mem::size_of::<u8>());
    println!("i16  size: {} bytes", mem::size_of::<i16>());
    println!("u16  size: {} bytes", mem::size_of::<u16>());
    println!("i32  size: {} bytes (default integer)", mem::size_of::<i32>());
    println!("u32  size: {} bytes", mem::size_of::<u32>());
    println!("i64  size: {} bytes", mem::size_of::<i64>());
    println!("u64  size: {} bytes", mem::size_of::<u64>());
    println!("i128 size: {} bytes", mem::size_of::<i128>());
    println!("u128 size: {} bytes", mem::size_of::<u128>());
    println!("usize size: {} bytes (depends on system)", mem::size_of::<usize>());
    println!("isize size: {} bytes (depends on system)", mem::size_of::<isize>());

    // ---------------------------------
    // 2️⃣ FLOATING POINT TYPES
    // ---------------------------------
    println!("\n-- Floating Point Types --");

    println!("f32 size: {} bytes", mem::size_of::<f32>());
    println!("f64 size: {} bytes (default float)", mem::size_of::<f64>());

    // ---------------------------------
    // 3️⃣ BOOLEAN
    // ---------------------------------
    println!("\n-- Boolean --");

    println!("bool size: {} byte", mem::size_of::<bool>());

    // ---------------------------------
    // 4️⃣ CHARACTER
    // ---------------------------------
    println!("\n-- Character --");

    let letter: char = 'A'; // must use single quotes
    println!("char size: {} bytes", mem::size_of::<char>());
    println!("Example char: {}", letter);

    // ---------------------------------
    // 5️⃣ TUPLE
    // ---------------------------------
    println!("\n-- Tuple --");

    let tuple: (i32, f64) = (20, 3.14);
    println!("Tuple size: {} bytes", mem::size_of_val(&tuple));

    // ---------------------------------
    // 6️⃣ ARRAY
    // ---------------------------------
    println!("\n-- Array --");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array size: {} bytes", mem::size_of_val(&arr));

    // ---------------------------------
    // 7️⃣ STRING TYPES
    // ---------------------------------
    println!("\n-- String Types --");

    println!("&str size: {} bytes", mem::size_of::<&str>());
    println!("String size (stack only): {} bytes", mem::size_of::<String>());

    println!("\n===== END =====");
}