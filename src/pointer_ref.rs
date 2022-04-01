// Reference pointer - point to a resource in memory
pub fn run() {
    // Primitive Array
    let arr1 = [3,4,5,6,6];
    let arr2 = arr1;
    println!("Primitive: {:?}", arr2);

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource. Vectors are non-premitives

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Non-premitive-1: {:?}", vec1);
    println!("Non-premitive-1: {:?}", &vec1);
    println!("Non-premitive-2: {:?}", vec2);
}