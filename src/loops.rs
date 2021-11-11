// loops - used to iterate until a condition ie met.

pub fn run() {

    let mut count = 0;

    // inifite loop 
    // (very specific loop in rust)
    println!("Loop State");
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20{
            break;
        }
    }

    // while loop (FizzBuzz)
    println!("Fizz Buzz");
    while count <= 100 {
        println!("{}", count);
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }

        count += 1;
    }

    // for range
    println!("For Loop");
    for x in 0..100 {
        let isEven = if x % 2 == 0 { "even" } else { "odd" };
        println!("{} is {}", x, isEven);
    }
}