use std::io;

fn main() {
    println!("_________________________________________________");
    println!("|   Menu                         |      Price   |");
    println!("|________________________________|______________|");
    println!("| Poundo, Yam/Edinkaiko Soup(1)  |     N3,200   |");
    println!("| Fried Rice & Chicken  (2)      |     N3,000   |");
    println!("| Amala & Ewedu Soup  (3)        |     N2,500   |");
    println!("| Eba & Egusi Soup    (4)        |     N2,500   |");
    println!("| White Rice & Stew    (5)       |     N2,500   |");
    println!("|________________________________|______________|");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What would you like to eat: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food:f32 = input1.trim().parse().expect("Not a valid string");

    println!("What quantity do you want:  ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f32 = input2.trim().parse().expect("Not a valid string");

    
    if food == 1.0 && quantity == 1.0{
        println!("Your bill = N3,200");
    }
    else if food == 1.0 && quantity > 1.0{
        let p = quantity * 3200.00;
        println!("Your bill = {}", p );
    }
    else if p > 10000.0 {fn p(){ 
        let pdiscount = p * 0.05;
        let pbill = p - pdiscount;
        println!("You have a discount of 5%...your new bill = {}",pbill );
    }}
    

    else if food == 2.0 && quantity == 1.0{
        println!("Your bill = N3,000");
    }
    else if food == 2.0 && quantity > 1.0{
        let F = quantity * 3000.0;
        println!("Your bill = {}", F );

    }
    else if F > 10000.0 {fn F(){
        let F = quantity * 3000.0;
        let Fdiscount = F * 0.05;
        let Fbill = F - Fdiscount;
        println!("You have a discount of 5%....your new bill = {}",Fbill );
    }}

    else if food == 3.0 && quantity == 1.0{
        println!("Your bill = N2,500");
    }
    else if food == 3.0 && quantity > 1.0{
        let a = quantity * 2500.0;
        println!("Your bill = {}",a );

    }
    else if a > 10000.0 {fn a() {
        let a = quantity * 2500.0;
        let adiscount = a * 0.05;
        let abill = a - adiscount;
        println!("You have a discount of 5%...your new bill = {}",abill );
    }}
    else if food == 4.0 && quantity == 1.0{
        println!("Your bill = N2,500");
    }
    else if food == 4.0 && quantity > 1.0{
        let e = quantity * 2500.0;
        println!("Your bill = {}",e );

    }else if e > 10000.0 {fn e() {
        let e = quantity * 2500.0;
        let ediscount = e * 0.05;
        let ebill = e - ediscount;
        println!("You have a discount of 5%...your new bill = {}", ebill);
    }}

    else if food == 5.0 && quantity == 1.0{
        println!("Your bill = N2,500");

    }
    else if food == 5.0 && quantity > 1.0{
        let w = quantity * 2500.0;
        println!("Your bill = {}", w );
    } 
    else if w > 10000.0{ fn w() {
        let w = quantity * 2500.0;
        let wdiscount = w * 0.05;
        let wbill = w - wdiscount;
        println!("You have a discount of 5%...your new bill = {}", wbill );
    
    }}

}
