
// Loops - Used to iterate until a conditions is met 

pub fn run () {

    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {} ", count);

        if count == 20 {
            break;
        } 
    }



    // While loop
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("Fizzbuzz")
    //     }else if count % 5 == 0 {
    //         println!("fizz")
    //     }else {
    //         println!("{}", count);
    //     }

    //     // increment
    //     count += 1;
    // }




    // for range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("Fizzbuzz")
        }else if x % 5 == 0 {
            println!("fizz")
        }else {
            println!("{}", x);
        }
    }
    

}