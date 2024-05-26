pub fn run () {
 // print to console  
 println!("Hello world!");

 // basic formatting 
 println!("Number {}", 1);
 println!("{}, is from {}", "Bard", "Mass");

 // positional arguments
 println!("{0} is from {1} like to {2}", "Bard", "mass", "code");

  // name arguments
  println!("{name} like to play {activity}", name = "Jhon", activity ="Baseball");

  // placeholder traits
  println!("Binary: {:b} hex: {:x} Octal: {:o}", 10,10,10);


  // placeholder for debug traits
  println!("{:?}", (12, true, "hello"));


  // basic math
  println!("10 + 10 = {}", 10 + 10);
  println!("10 - 10 = {}", 10 - 10);
  println!("10 * 10 = {}", 10 * 10);
  println!("10 / 10 = {}", 10 / 10);
  println!("10 % 10 = {}", 10 % 10);
}

