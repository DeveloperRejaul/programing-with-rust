
use std::mem;
pub fn run () {
    let mut number:Vec<i32> = vec![1, 2, 3, 4, 5]; // heep allocated, dynamic array

    // Re-assign value 
    number[2]=20;


    // add on to vector
    number.push(10); 
    number.push(11); 

    // pop of last value 
    number.pop();

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


    // loop through vectors values 
    for x in number.iter(){
        println!("Numbers: {} ", x);
    }


    // loop and mutate values 
    for x in number.iter_mut(){
       *x *= 2
    }

    println!("Numbers Vec: {:?}", number);

}