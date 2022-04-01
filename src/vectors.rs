// Vectors - Resizable arrays
pub fn run() {
    let mut numbers: Vec<i32> = vec![5,6,7,8,4,9,5,6,1];

    //re-assign value
    numbers[5] = 14;
    println!("{:?}",numbers);

    // psuh value
    numbers.push(4);
    println!("{:?}", numbers);

    numbers.push(10);
    println!("{:?}", numbers);

    //pop value
    numbers.pop();
    println!("{:?}", numbers);

    let removed = numbers.pop();
    println!("{:?} and the array is {:?}", removed, numbers);

    // get single value
    println!("The second value is - {}", numbers[1]);

    // slice vector
    let slice:&[i32] = &numbers[2..7];
    println!("The sliced vector is - {:?}", slice);

    let length = numbers.len();
    println!("The length is - {}", length);

    // Vectors are heap allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Loop thorugh vector values
    for x in numbers.iter() {
        println!("The num is-{}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Every element will be multyplied with 2 -- {:?}", numbers);
}