pub fn run () {
    let array = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    println!("{:?}", array);

    let new_array = array.map(|x| x+1);
    println!("{:?}", new_array);
}