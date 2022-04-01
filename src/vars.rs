// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Habib";

    // assign mutable variable
    let mut age = 19;
    println!("My name is {}, my age is {}", name, age);

    // change variable value
    age = 20;
    println!("My name is {}, my age is {}", name, age);

    // define constant
    const ID: i32 = 002;
    println!("The id is - {}", ID);

    // Assign multiple variable
    let (my_name, my_age) = ("Habibul Islam", 19);
    println!("{} is {}", my_name, my_age);
}