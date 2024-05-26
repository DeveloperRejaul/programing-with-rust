pub fn run () {
    let name = "Bard";
    let mut age = 20;
    
    println!("My name is {} and i am {}", name, age);
    
    age = 40;
    println!("My name is {} and i am {}", name, age);


    // defile const
    const ID : i32 = 001;
    println!("ID: {}", ID);


    // assign multiple variables
    let (my_name, my_age) = ("Bard", 37);
    println!("{} is {}", my_name, my_age)

}