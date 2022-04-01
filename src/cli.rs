use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    
    if command == "status" {
        println!("Status = 100%");
    } else if command == "start" {
        println!("Started");
    }
    else if command == "stop" {
        println!("Stopped");
    } else {
        println!("Command not found");
    }
}