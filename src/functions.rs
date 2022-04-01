// Functions - Used to store blocks of code for re-use
pub fn run() {
    sum(5, 2);

    // get return value
    let get_sum = sum_return(2, 10);
    println!("{}",get_sum);

    // closure
    let n3: i32 = 100;
    let total_price = | n1: i32, n2: i32| n1 + n2 + n3;
    println!("Total price: {}", total_price(200, 200));

}

// simple function
fn sum(n1:i32, n2:i32) {
    println!("{}", n1+n2);
}

// return
fn sum_return(n1: i32, n2: i32) -> i32 {
    n1 + n2
}