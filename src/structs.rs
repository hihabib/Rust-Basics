// Structs - Used to create custom data types

// treditional structs
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct Person
    fn new(fname:&str, lname:&str) -> Person {
        Person {
            first_name: fname.to_string(),
            last_name: lname.to_string()
        }
    }

    // get full name
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // get first name
    fn get_first_name(&self) -> String {
        format!("{}", self.first_name)
    }

    // get last name
    fn get_last_name(&self) -> String {
        format!("{}", self.last_name)
    } 

    // to tuple
    fn to_tople(&self) -> (String, String){
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

// Touple struct
struct Color (u8, u8, u8);




pub fn run() {
    // Treditional struct
    let mut person = Person {
        first_name: "Habibul".to_string(),
        last_name: "Islam".to_string()
    };

    person.first_name = "Md".to_string();
    person.last_name = "Habib".to_string();

    println!("My name is {} {}", person.first_name, person.last_name);

    // touple struct
    let color = Color (255, 0, 0);
    println!("My fevourite color is: rgb({},{},{})", color.0, color.1, color.2);

    // call with impl fn constructor
    let my_name = Person::new("Habibul", "Islam");
    println!("My Full name is {} {}", my_name.first_name, my_name.last_name);

    // call with impl fn full_name
    println!("My Full name is {}", my_name.get_full_name() );

    // call with getter
    println!("My first name is {}", my_name.get_first_name());
    println!("My last name is {}", my_name.get_last_name());

    // with tuple
    println!("I am Habib ({:?})", my_name.to_tople());
}