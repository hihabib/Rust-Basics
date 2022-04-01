// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, String, i8) = ("Habib", "Noakhali".to_string(), 19);
    println!("I'm {}, from {}. I am {} years old", person.0, person.1, person.2);
}