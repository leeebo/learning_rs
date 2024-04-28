// import the io module from the standard library
// module is a collection of items: functions, structs, traits, etc.
use std::io;
// import the Rng trait from the rand crate
// Crate is external package that are not part of the standard library
// Q: what is a trait?
use rand::Rng;
// import the Ordering enum from the standard library
use std::cmp::Ordering;

fn main() {
    // println!, the ! means it's a macro
    println!("Guess the number!");
    // declare a constant variable
    // using random generator to generate
    // using range expression start..=end to limit the range
    let random_number = rand::thread_rng().gen_range(1..=100);
    let max_guesses = 6;
    let mut guesses = 0;

    loop {
        println!("Please input your guess.");
        
        // declare a mutable variable
        // String::new() is a static method that returns a new empty string
        let mut guess = String::new();
        
        // io::stdin() returns an instance of std::io::Stdin
        // read_line() is a method of std::io::Stdin
        // read_line() takes a mutable reference to a string
        // expect() is a method of Result
        // expect() takes a string as an argument
        // expect() is called when the Result is an Err
        // expect() will crash the program and display the message
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        // shadowing the guess variable
        // trim() method eliminates whitespace and newlines from the string
        // parse() method parses a string into a number
        // parse() returns a Result type
        // expect() is called when the Result is an Err
        // expect() will crash the program and display the message
        // if the Result is an Ok, the value will be assigned to guess
        // the : i32 annotation tells Rust to infer the type of guess as i32
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catchall value
            Err(_) => continue,
        };
        // difference kind of println
        println!("You guessed: {}", guess);
        // println!("The random number is: {random_number}");
        
        // the match expression is made up of arms
        // each arm has two parts: a pattern and some code
        // the cmp() method compares two values and returns an Ordering
        // the match expression compares the result of cmp() to each arm's pattern
        match guess.cmp(&random_number) {
            // Ordering is an enum with three variants: Less, Greater, Equal
            // if the result of cmp() is Ordering::Less, the code in the arm will run
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!");
                break;
            },
        }

        guesses += 1;
        if guesses == max_guesses {
            println!("You lose! The number was: {}", random_number);
            break;
        }
    }
}
