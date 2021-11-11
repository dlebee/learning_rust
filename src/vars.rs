// Variables hold primitive data or references to data
// Variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "David";
    let mut age = 32;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // define constant (constants must be typed.)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let ( my_name,my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}