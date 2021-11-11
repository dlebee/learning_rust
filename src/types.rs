

pub fn run() {

    // by default its i32 
    let x = 1; 
    
    // by default its a float64
    let y = 2.5; 

    // add explicit type 
    let z: i64 = 12312312312313;

    // find max value of i64
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // bolean
    let is_active: bool = true;

    // get booleans from epression
    let is_greater = 10 > 5;

    // char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}