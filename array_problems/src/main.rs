fn main () {
    let array = [1,2,3,4,5,6,7,8,9,10];

    let res = array.binary_search(&7);

    println!("{:?}" ,array);
    println!("{:?}",res);
}