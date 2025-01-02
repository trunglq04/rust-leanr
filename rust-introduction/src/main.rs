
fn main() {
    // VARIABLES AND DATATYPES
    let mut x: i32 = 5;
    x = x + 1;

    let y = 10;

    println!("Giá trị biến x: {1}, Giá trị của biến y : {0}", x, y);
    println!("Giá trị biến x: {x}, Giá trị của biến y : {y}");

    let bootcamp = "Rust Bootcamp";
    let year = 2024;
    let free = true;
    let z = 10.0;
    let mut zz = 'b';
    zz = 'c';
    // i32 -> integer 32 bit -> both pos and neg
    // u32 -> integer 32 bit -> only pos
    // const mut PI = 3.14; -> err -> global constants cannot be mutable

    // all variable must be same data-type
    let array = [1u32, 2, 3, 4];

    let first = array[0];
    // let error = array[5];
    println!("First: {}", first);
    // println!("Error: {}", error);

    // TYPE CASTING
    let x: u16 = 256;
    let y = x as u8;
    println!("x: {x}"); // Result: 256
    println!("x as u8: {y}"); // Result: 0

    let character = 'A';
    let integer = character as u8;
    println!("character as u8: {integer}");

    let free = true;
    let free_integer = free as u8;
    println!("true as u8: {free_integer}");

    // OPERATORS
    let a = 10;
    let b = 20;

    let add = a + b;
    let sub = a - b;
    println!("add: {}", add);
    println!("sub: {}", sub);

    let mut x = 10;
    x += 1;
    println!("x: {x}");

    let c = a < b;
    let d = a == b;
    println!("a > b: {c}");
    println!("a == b: {d}");

    let z = c && d;
    let w = c || d;
    let q = !x;
    println!("Z: {z}");
    println!("W: {w}");
    println!("Q: {q}");

}
