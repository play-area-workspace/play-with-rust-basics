use std::any::type_name;

fn main() {
    println!("Hello, Rust 🦀 from CARGO!");

    let x = 5; // rust infers the type here to default int type which is i32
    println!("The value of x is: {}, and type is: {}", x, type_of(&x));

    let y: i64 = -60;
    println!("The value of y is: {}, and type is: {}", y, type_of(&y));

    let letter: char = 'a';
    println!(
        "The letter is: {}, and type is: {}",
        letter,
        type_of(&letter)
    );

    let pi: f64 = 3.14;
    let is_floated: bool = true;

    println!("values {}, {}", pi, is_floated);

    // Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of numbers is: {:?}", numbers);

    let fruits: [&str;3] = ["apple", "banana", "orange"];
    println!("Fruit Array: {:?}",fruits);
    println!("1st element of Fruit Array: {}", fruits[0]);
    println!("2nd element of Fruit Array: {}", fruits[1]);
    println!("3rd element of Fruit Array: {}", fruits[2]);
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}