use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Are you experienced: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:bool = input1.trim().parse().expect("Not a valid string");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid string");

    if experience == true && age >= 40.0{
        println!("Your incentive = 1,560,000");
    }
    else if experience == true && age >= 30.0 && age < 40.0{
        println!("Your incentive = 1,480,000");
    }
    else if experience == true && age < 28.0{
        println!("Your incentive = 1,300,000 per month");
    }
    else if experience == false{
        println!("Your incentive = 100,000");
    }
}