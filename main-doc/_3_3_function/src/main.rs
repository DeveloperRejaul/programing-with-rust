

//TODO!In rust main is entry function like js window

fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);
    let x = five();
    let y = plus_one(5);
    let my_str = stringFn();
    let c=  process_data(2,callback_function) ;


    // rest parameters
    let  num = [1, 2, 3, 4, 5];
    let result = sum(&num);
    println!("Sum: {}", result);


    print!("value: {}", x);
    println!("value: {}", y);
    println!("{}", my_str);
    println!("{}", c);
}


//TODO:void function
fn another_function() {
    println!("Another function");
}


//TODO:function with parameter 
fn another_function2 (x: i32) {
    println!("Another function2 {x}")
}

fn callback_function(x: i32) -> i32 {
    x * 2
}

// callback function parameter
fn process_data(x:i32, callback:fn(i32)->i32) -> i32 {
    let y = callback(5);
    y+x
} 

// rest parameters
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}


//TODO:Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
     x + 1
}
fn stringFn () -> String {
    let myString = String::from("Hello world!");
    myString
}
