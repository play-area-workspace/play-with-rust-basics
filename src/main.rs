use std::any::type_name;

fn main() {
    println!("Hello, Rust 🦀 from CARGO!");

    let x = 5;   // rust infers the type here to default int type which is i32
    println!("The value of x is: {}, and type is: {}", x, type_of(&x));

    let y:i64 = -60;
    println!("The value of x is: {}, and type is: {}", y,type_of(&y));
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}