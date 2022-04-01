// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // let text = "Hello ";
    // let mut str = String::from(text);
    let mut str = String::from("Hello ");

    println!("Length: {}",  str.len());

    // push char
    str.push('W');

    //push string
    str.push_str("orld");
    // print
    println!("{}", str);

    // capacity
    println!("The capacity is {}", str.capacity());

    // is_empty
    println!("Is empty: {}", str.is_empty());

    // contains
    println!("Is contain 'world' -  {}", str.contains("World"));

    // replace
    println!("Replaced - {}", str.replace("World", "there"));

    // Loop through string by whitespace
    for word in str.split_whitespace() {
        println!("The word is {}", word);
    }

    // create string with capacity
    let mut _str = String::with_capacity(5);
    _str.push_str("Hello");
    println!("{}", _str.capacity());

    // assertion testing
    assert_eq!(5, _str.capacity());
    assert_eq!(5, _str.len());
}