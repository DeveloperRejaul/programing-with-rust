
// Array - Fixed list where elements are the same data type
use std::mem;
pub fn run () {
    let mut number:[i32;5] = [1, 2, 3, 4, 5];

    // Re-assign value 
    number[2]=20;

    // print number
    println!("{:?}", number);


    // get single value from array
    println!("Single value: {}", number[0]);

    // Get array len 
    println!("Array length : {} ", number.len());


    // Array are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&number));

    // get slice 
    let slice:&[i32]= &number[0..2];
    println!("Slice {:?}", slice);
}