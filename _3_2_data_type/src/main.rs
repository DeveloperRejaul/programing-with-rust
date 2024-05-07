fn main() {
    // TODO:Integer Types
    // Length	Signed	    Unsigned
    // 8-bit	i8	        u8
    // 16-bit	i16 	    u16
    // 32-bit	i32	        u32
    // 64-bit	i64	        u64
    // 128-bit	i128	    u128
    // arch	    isize	    usize

   
    //  *difference between signed and unsigned
    //  !Signed variable e negative number and positive number store kura jay
    //  !unsigned variable e sodo positive number store kura jay
   

    // Example: Signed 
    let num:i32 = -10;
    let num2:i32 = 10;
    println!("{}", num);
    println!("{}", num2);

    // Example: unsigned 
    let num3:u32 = 10;
    println!("{}", num3);





    //TODO:Floating-Point Types
    // 2 type 1.f23, f64
    let x = 2.0; // f64  default
    let y: f32 = 3.0; // f32

    println!("{} {}", x,y);


    //TODO:The Tuple Type
    let tup:(i32, f64, i8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{} {} {}", five_hundred,six_point_four,one);



    //TODO:The Array Type
    // !every element of an array must have the same
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    // !!Accessing Array Elements
    let first = a[0];
    let second = a[1];

    // print array data
    let formatted_array = format!("{:?}", months);
    println!("{}",formatted_array );
}
