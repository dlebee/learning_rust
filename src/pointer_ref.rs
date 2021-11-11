// reference pointeres - point to a resource in memory

pub fn run() {

    // primitive array
    let arr1 = [1, 2, 3,];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));


    // with non primtives if you assign another variables 
    // to a piece of data, the first variable will no longer 
    // hold that value, you'll need to use a refernec (&) 
    // to point to the resource

    // Vector
    let vec1 = vec![1, 2, 3,];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}