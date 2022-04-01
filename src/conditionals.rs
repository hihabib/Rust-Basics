pub fn run() {
    let age = 19;
    let id = true;
    if id && age >= 20 {
        println!("You should study in CSE")
    } else if id && age <= 20 && age >= 17 {
        println!("It's your admission time. Do academic study");
    } else if !id {
        println!("Collect your id")
    } else {
        println!("Learn new programming for getting knowledge");
    }
}