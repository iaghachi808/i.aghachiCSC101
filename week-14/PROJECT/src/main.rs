use std::io;
use std::io::Read;


fn main() {
    println!("WELCOME TO GLOBACOM DATABASE MANAGEMENT SYSTEM");

      administrator();

     
      projecymanager();

      stafftable();
      customertable();

      vendor();


}
 
fn administrator() {
     let mut trativeuser = String::new();
println!("Are you and administrator, If yes enter password");
    io::stdin().read_line(&mut trativeuser).expect("Not a valid string");
    let trativeuser:bool = trativeuser.trim().parse().expect("Not a valid string ");

if trativeuser == true {
            let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    }

    else {
                      println!("SORRY YOU'RE NOT ALLOWED TO VIEW THE DATABASE BECAUSE YOU ARE NOT AN ADMINISTRTOR BUT IF YOU'RE A PROJECT MANAGER YOU CAN VIEW THE NEXT ONE");

    } 

 

}

fn projecymanager() {
     let mut pmanager = String::new();
println!("Are you a Project Manager, If yes enter password");
    io::stdin().read_line(&mut pmanager).expect("Not a valid string");
    let pmanager:bool = pmanager.trim().parse().expect("Not a valid string ");

    if pmanager == true{
        let mut file = std::fs::File::open("PROJECT_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else {
                              println!("SORRY YOU'RE NOT ALLOWED TO VIEW THE DATABASE BECAUSE YOU ARE NOT A PROJECT MANAGER BUT IF YOU'RE A STAFF YOU CAN VIEW THE NEXT ONE");

        
    }

}

fn stafftable() {
    let mut staff = String::new();
println!("Are you an Employee, If yes enter password");
    io::stdin().read_line(&mut staff).expect("Not a valid string");
    let staff:bool = staff.trim().parse().expect("Not a valid string ");

    if staff == true{
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else {
                              println!("SORRY YOU'RE NOT ALLOWED TO VIEW THE DATABASE BECAUSE YOU ARE NOT AN EMPLOYEE BUT IF YOU'RE A CUSTOMER YOU CAN VIEW THE NEXT ONE");

    }

}

fn customertable() {
    let mut customer = String::new();
println!("Are you an customer, If yes enter password");
    io::stdin().read_line(&mut customer).expect("Not a valid string");
    let customer:bool = customer.trim().parse().expect("Not a valid string ");

    if customer == true{
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else {
                              println!("SORRY YOU'RE NOT ALLOWED TO VIEW THE DATABASE BECAUSE YOU ARE NOT A CUSTOMER BUT IF YOU'RE A VENDOR YOU CAN VIEW THE NEXT ONE");



    }

}

fn vendor() {
    let mut vendord = String::new();
println!("Are you an Vendor, If yes enter password");
    io::stdin().read_line(&mut vendord).expect("Not a valid string");
    let vendord:bool = vendord.trim().parse().expect("Not a valid string ");

    if vendord == true{
        let mut file = std::fs::File::open("DATA_PLAN_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    } else {
        println!("YOU'RE JUST USELESS");


    }

}

