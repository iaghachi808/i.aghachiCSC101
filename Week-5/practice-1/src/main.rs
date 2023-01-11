use std::io;

fn main() {
    println!("\n Student Information Management System!");

    // input name
    println!("\nPlease enter your name.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Your name is: {}");
    println!("Your name is: {}", name);
    
    // input age
    println!("\nEnter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);

            
}
