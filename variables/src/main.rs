fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("{}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    // Data Types:

    // Integers
    let a = 98_222; // Decimal
    let b = b'A'; // Byte
    
    // Floating Point
    let f = 2.0;

    let t = true;


    // Compound Types
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];


    // Functions in Rust:
    
}
