pub fn run () {
    let age = 18;
    let check_id = false;
    let knows_person_of_age=true;

    if age >= 21 {
        println!("Bartends : what would you like to drink ?");
    }else {
        println!("Bartends : Sorry , you have to leave");
    }

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartends : what would you like to drink ?");
    }else if age < 21 && check_id{
        println!("Bartends : Sorry , you have to leave");
    }else {
        println!("Bartends : I'll need to see your ID");
    }


    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is_of_age: {}", is_of_age);

}