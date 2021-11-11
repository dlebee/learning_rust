// Structs -used to create custom data types

// Traditional S truct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

// struct with functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }


    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!(
        "Color: {} {} {}", 
        c.red, c.green, c.blue
    );

    let mut tuple_c = ColorTuple(255, 0, 0);
    tuple_c.0 = 200;
    println!("Color 2: {} {} {}", 
        tuple_c.0, tuple_c.1, tuple_c.2
    );

    let mut p = Person::new("Eliana", "Shore");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());
    p.set_last_name("Lebee");
    println!("Person married so new name is: {}", p.full_name());
    println!("Person tuple {:?}", p.to_tuple());
}