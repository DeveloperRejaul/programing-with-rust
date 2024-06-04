pub fn run () {
    let hello2 = "hello world"; // stack allocated string , not changeable 
    let mut hello:String = String::from("Hello "); // hep allocated string, dynamic string changeable

    // get length 
    println!("Length: {}", hello.len());

    // push char
    hello.push('w');

    // push string 
    hello.push_str("orld");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());


    // check empty
    println!("Is empty: {} ", hello.is_empty());

    // contains 
    println!("Contains world: {}", hello.contains("world"));

    // replace
    println!("Replace : {}", hello.replace("world", "There"));


    // loop through string by whitespace
    for word in hello.split_whitespace()  {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}",s);

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    // print 
    println!("{}", hello);   
}