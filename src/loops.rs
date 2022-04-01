// Loops - Used to iterate until a condition is met
pub fn run() {

    //infinite loop

    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("{}", count);
    // }

    // loop with condition to stop it

    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("{}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop with FizzBuzz

    // let mut count = 0;
    // while count < 100 {
    //     if count % 15 == 0 {
    //         println!("Fizz");
    //     } else if count % 3 == 0 {
    //         println!("Buzz");
    //     } else if count % 5 == 0 {
    //         println!("FizzBuzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    // For Rage with "Fizzbuzz"
    for x in 0..100 {
        if x % 15 == 0 {
            println!("Fizz")
        } else if x % 3 == 0 {
            println!("Buzz");
        } else if x % 5 == 0 {
            println!("Fizzbuzz");
        } else {
            println!("{}", x);
        }
    }
}
