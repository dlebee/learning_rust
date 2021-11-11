pub fn run() {
    // str (constant fixed-length immutable)
    let immutableHello = "hello";

    // String growable heap allocated data structure
    let mut hello = String::from("Hello ");

    // push only add one character (since string is an array of chars)
    hello.push('W');

    // how to push a literal string.
    hello.push_str("orld!");

    // get length
    println!("Length: {}", hello.len());

    // capacity in memory
    println!("Capacity: {}", hello.capacity());

    // is empty
    println!("is empty: {}", hello.is_empty());

    // contains.
    println!("Contains 'World': {}", hello.contains("World"));

    // replace.
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace.
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with capacity.
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{:?} | length {} | capacity {}", s, s.len(), s.capacity());

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}