// Rust program to determine age pass

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid string");

    if age >= 18.0{
        println!("Welcome to the party {}!", input1);
    } else {
        println!("OOPs you're too small {}!", input1);
    } if age > 30.0{
        println!("You're too old for this {}", input1);
    }
}