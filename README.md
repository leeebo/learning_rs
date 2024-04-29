# Learning Rust with C basics

## Hello, World!

```rust
fn main() {
    println!("Hello, World!");
}
```

1. `fn` keyword is used to declare a function
2. `main` function is the entry point of the program
3. `!` indicates that `println` is a macro, not a function
4. `println!` is a macro that prints text to the console

## Variables and Mutability

```rust
let x = 5;
let y: i32 = 10;
let mut z = 15.0;
let y = "Hello, World!"; // shadowing
```

1. using `let` to declare a variable
2. using `:` after the variable name to specify the type of the variable (optional)
3. variables are immutable by default
4. using `mut` to make a variable mutable
5. variables can be shadowed

## Data Types

```rust
let x = 0b1101_1010;
let c = '😆';
let t = (1, 2.0, "three");
// same with let t: (i32, f32, &str) = (1, 2.0, "three");
let a = [1, 2, 3, 4, 5];
```

1. `_` can be used as a visual separator for large numbers
2. `char` type represents a single Unicode character, which is 4 bytes in size
3. `tuple` type can be used to group multiple values of different types
4. `array` type has a fixed length and all elements must have the same type

> `index out of bounds` error will be thrown at runtime if the index is out of bounds

## Functions

```rust
fn addi32(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 5;
    let y = 10;
    let z = addi32(x, y);
    println!("{} + {} = {}", x, y, z);
}
```

1. functions are declared using the `fn` keyword
2. function's parameters must have type annotations, eg. `x: i32`
3. function's return type using annotations, eg. `-> i32`
4. statements not returning a value, `x=y=5;` is not allowed
5. expressions do not include ending semicolons, `x+y` is an expression, `x+y;` is a statement