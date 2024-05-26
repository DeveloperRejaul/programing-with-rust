/*
Primitive Types--
Integers: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 (Number of bytes taken memory)
Float: f32, f64,
Boolean: bool,
Character: char,
Tuples: (i32, f64, u8),
Arrays: [i32; 5],
*/ 

// rust is statically typed language, which means that is must know the types of all variables at compile time, 
// however compiler can usually infer what types we want to use based on the value and how we use it.




pub fn run () {

    // default is "i32"
    let x =1; 

    // default is "f64"
    let y = 2.5;

    // add explicit type
    let z:i64 = 21412341234;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);

    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);


    // Boolean
    let active = true;

    // get boolean from expression
    let is_greater = 10 > 5;

    // char
    let a1 = "a";
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z, active,is_greater, a1,face ))
}