use std::io;

fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let _i: u32 = 10;
    let _f: f32 = 10.0;
    let _b: bool = true;
    let _c: char = 'z';
    let _k: char = '캌';
    let _e: char = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (x, y, z) = tup; // destructuring
    println!("x: {x}, y: {y}, z: {z}");

    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    let _a = [3; 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
