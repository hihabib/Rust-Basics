/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.


pub fn run() {
    // default it is "i32"
    let x  = 1;
    println!("this is {}", x);

    // default it is "f64"
    let y = 1.2;
    println!("it is {}", y);

    // add explicit type
    let z: i64 = 34343243243242;
    println!("it is {}",z);

    // get max size
    println!("Max size of i32 is - {}", std::i32::MAX);
    println!("Max size of i64 is - {}", std::i64::MAX);
    println!("Max size of i128 is - {}", std::i128::MAX);

    // boolean
    let is_active: bool = true;
    println!("this is - {}", is_active);

    // get boolean from expression
    let is_true = 10 < 3;
    println!("it is - {:?}", (is_true, is_active, z));

    // Char
    let a = 'A';
    println!("The character is - {}", a);

    let emoji = '😋';
    println!("the emoji is - {}", emoji);

    let unicode_emoji = '\u{1F600}';
    println!("The smile is - {}", unicode_emoji);

}