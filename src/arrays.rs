// arrays are fixed length and must be the same data types.

// by doing this you don't have to write std::mem::size_of_val(..);
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [
        1, 2, 3, 4, 5
    ];

    println!("Debug Print: {:?}", numbers);

    // if its mutable you can replace values at index.
    numbers[2] = 20;

    // single value
    println!("Single index print: {}", numbers[0]);

    // length
    println!("length of array is: {}", numbers.len());

    // memory usage (stack allocated)
    // similar to C/C++ its by reference using &
    println!("memory used by array: {} bytes", mem::size_of_val(&numbers));

    println!("For Each loop");
    for number in numbers {
        println!("{}", number);
    }

    // get slices of array
    let slice: &[i32] = &numbers[0..2];
    // you don't really need to write all this 
    // so i am adding the short hand version of it.
    let slice_shorter = &numbers[0..2];
    println!("Slice: {:?} {:?}", slice, slice_shorter);
}