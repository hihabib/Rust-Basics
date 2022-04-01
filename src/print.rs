pub fn habib() {
    // Printing
    println!("This is printing");

    // Basic Formating
    println!("This is {}, and my age is {}", "Habib", 19);

    // Positional Arguments
    println!(
        "{0} is a boy, he likes {1}. Generally {0} don't like {2}. Always he like to do {1}",
        "habib", "coding", "study"
    );

    // Named Arguments
    println!(
        "{name} is a good boy. He likes {programming}. He is {age} years old",
        name = "Habibul Islam",
        programming = "Rust",
        age = 19
    );

    // placeholder traits
    println!("binary- {:b}, octal- {:o}, hex: {:x}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "Hello world"));

    // basic math
    println!("10 + 10 = {0}", 10+10);
}
