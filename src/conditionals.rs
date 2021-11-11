

pub fn run() {

    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // there are no paranthesis required in rust compared
    // to other C Styled Languages
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what would you like to drink?");
    } else if (age < 21 && check_id) { 
        // but you can use them, just get a warning.
        println!("Bartender: Too young to drink, sorry.");
    } else {
        println!("Bartender: I'll need to see your id");
    }

    // there are no ternary operators in rust
    // but you can use short hand equations
    // which is is very similar.
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}