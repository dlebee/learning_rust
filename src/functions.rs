// functions - meant to store logical code for reusability.

pub fn run() {
    greeting("Hello", "David Lebee");

    // bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure
    // closure have the ability to use variables in the current scope.
    let n3: i32 = 10;
    let add_nums = |l: i32, r: i32| l + r + n3; // <-- see
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}