// Function use to store block of code for re-use

pub fn run () {

    greeting("Hello", "Rezaul");
    println!(" 10 + 10 = {}",add(10, 10)) ;

    // Closure 
    let n3:i32 = 10;
    let add_num = | n1:i32, n2:i32 | n1+n2 + n3;
    println!("6 + 6 + n3 = {}", add_num(6,6)) ;

}


fn greeting (greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}


fn add(num1:i32, num2:i32)-> i32 {
    num1 + num2
}


