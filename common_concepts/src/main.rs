use std::io;

fn main() {
    // char is four bytes in size
    let c = '嗨';
    println!("{c}, world!");
    // _ can be used to improve readability
    let num = 0b1010_1010;
    println!("{num}");

    // the tuple is a compound data type
    let data: (i32, f64, u8) = (500, 6.4, 1);
    // the destructuring of a tuple
    let (x, y, z) = data;
    println!("The value is: {}, {}, {}", x, y, z);
    // the access to a tuple element
    println!("The value is: {}, {}, {}", data.0, data.1, data.2);

    // the array is a fixed-size list of elements of the same type
    // every element of the array must have the same type
    // same as let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    // same as let b = [3, 3, 3, 3, 3];
    let b = [3; 5];
    println!("The value is: {}, {}, {}", a[0], a[1], a[2]);
    println!("The value is: {}, {}, {}", b[0], b[1], b[2]);

    // the access to an array element,
    // error will be thrown if the index is out of bounds
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u32 = user_input.trim().parse().expect("Please type a number");
    println!("You typed: {}", a[user_input as usize]);
}
