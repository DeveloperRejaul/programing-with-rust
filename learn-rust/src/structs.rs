
// Structs - Used to create custom data type


// Traditional Struct
struct Color {
    red:u8,
    green:u8,
    blue:u8, 
}

struct  Person {
    first_name: String,
    last_name:String,
}


// like class
impl  Person {
    // construct person 
    fn new(first:&str, last:&str) -> Person {
        Person {
             first_name: first.to_string(),
             last_name:last.to_string() 
        }
    }

    // get full name 
    fn full_name (&self)-> String {
       format!("{} {}", self.first_name, self.last_name)
    }
    // set last name
    fn set_last_name (&mut self, last:&str) {
        self.last_name = last.to_string();
    } 

    
   // name to tuple
   fn to_tuple (self)-> (String, String) {
    (self.first_name ,self.last_name)
   }

}

// Tuple Struct 
struct Color2 (u8, u8,u8);

pub fn run () {

   let mut c = Color{
    red:225,
    green:0,
    blue:0
   };

   // change value 
   c.red = 200;
   println!("Color: {} {} {}", c.red, c.green, c.blue);

   let mut c2 = Color2(255, 0,0 );
   // change value
   c2.0 = 200;
   println!("Color : {} {} {}", c2.0, c2.1, c2.2);

   let mut p = Person::new("John", "Doe");

   println!("Person {} {}", p.first_name, p.last_name);
   
   println!("Person Full Name:  {}", p.full_name());

   p.set_last_name("Mia");

   println!("Person Full changed Name: {}", p.full_name());

   println!("Person Tuple: {:?}", p.to_tuple())
 
}