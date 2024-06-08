fn main() {
  
  //================================================================
  //                      handle condition
  //================================================================
    let i = 4;
    if i == 5 {
        println!("hello world!")
    }else {
        println!("goodbye world!")
    }

    // sort like js ternary operator
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    //================================================================
    //                      handle loop
    //================================================================
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    
    //================================================================
    //                      handle inner loop
    //================================================================
    let mut count = 0;
    let mut remaining = 10;

     loop {
        println!("count = {count}");
        loop {
            println!("remaining = {remaining}");
            if remaining == 5 {
                break;
            }
            remaining -= 1;
        }

        if count == 10 {
            break;
        }
        count += 1;
    }
    println!("End count = {count}");

    //================================================================
    //                      handle while loop
    //================================================================
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //================================================================
    //                      handle for loop
    //================================================================
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for num2 in (1..4).rev() {
        println!("{num2}!");
    }
    println!("LIFTOFF!!!");



}
