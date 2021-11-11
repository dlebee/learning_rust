// arrays are fixed length and must be the same data types.

// by doing this you don't have to write std::mem::size_of_val(..);
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("Debug Print: {:?}", numbers);

    // if its mutable you can replace values at index.
    numbers[2] = 20;

    // add on to the vector
    numbers.push(5);
    numbers.push(6);

    println!("Debug Print: {:?}", numbers);

    // pop
    numbers.pop(); //remove last.

    println!("Debug Print: {:?}", numbers);

    // single value
    println!("Single index print: {}", numbers[0]);

    // length
    println!("length of vector is: {}", numbers.len());

    // memory usage (stack allocated)
    // similar to C/C++ its by reference using &
    println!("memory used by vector: {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    // loop trhough vector value.
    println!("Printing one at a time with a for each.");
    for x in numbers.iter() {
        println!("{}", x);
    }

    // loop trhough vector value.
    println!("Loop and mutate value");
    for x in numbers.iter_mut() {
        // look at the *x before its pretty much meaning touch
        // the value like in C pointers
        *x *= 2; // this is like C
    }
    println!("Debug Print: {:?}", numbers);
}