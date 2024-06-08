// reverse array 
pub fn run () {
    let arr = vec![1,2,3,4,5,6,7];
   
    // reverse array with own method
    let mut arr2 = arr.clone();
    arr2.reverse();
    println!("{:?}", arr2);
 
   // reverse array with custom function
   let new_array =  reverse_array(&arr);
   println!("{:?}", new_array);

   let new_array2 =  reverse_array2(&arr);
   println!("{:?}", new_array2);
}


fn reverse_array (arr: &Vec<i32>) -> Vec<i32>   {
    let mut new_array: Vec<i32> = vec![];
    let mut count = arr.len();


    while count > 0 {
        count = count - 1;
        new_array.push(arr[count]);
    }

    return new_array;
}


fn reverse_array2 (arr: &Vec<i32>) -> Vec<i32>   {
    let mut new_array: Vec<i32> = vec![];
   
   for elements in arr.iter().rev() {
       new_array.push(*elements); // dereference
   }
    return new_array;
}