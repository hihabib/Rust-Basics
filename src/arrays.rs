// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let mut numbers: [i32; 7] = [9,10,11,12,13,14,15];

    // print full array
    println!("{:?}", numbers);

    // print single value
    println!("{}", numbers[1]);

    // change the value
    numbers[1] = 18;
    println!("{}", numbers[1]);
    println!("{:?}", numbers);

    // array length
    println!("The length is - {}", numbers.len());

    // array are stack allocated
    println!("{}", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..6];

    println!("{:?}", slice);
}