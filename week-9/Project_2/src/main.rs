use std::io::Write;
use std::io::Read;

fn main() {
    let student_name = vec!["Oluchi Mordi  ","Adams Aliyu  ","Shania Bolade  ","Adekunle Gold  ","Blanca Edemoh  "];
    let matric_number = vec!["ACC10211111  "," ECO10110101  ","CSC10328828  ","EEE11020202  ","MEE10202001  "];
    let Department = vec!["Acconting  "," Economics  ","Computer  ","Electrical  ","Mechanical  "];
    let Level = vec!["300  "," 100  ","200  ","200  ","100  "];

    let mut file = std::fs::File::create("PAU_SIMS.txt").expect("create failed");
    file.write_all(b"Student Name  |  Matric Number  |  Department  |  Level  |",);
    file.write_all(b"\n");
    file.write_all(b"| ________________________________________________________|");

    for i in 0..student_name.len(){
        file.write_all("\n".as_bytes()).expect("create failed");
        file.write_all(student_name[i].as_bytes()).expect("create failed");
        file.write_all(" | ".as_bytes()).expect("create failed");
        file.write_all(matric_number[i].as_bytes()).expect("create failed");
        file.write_all(" | ".as_bytes()).expect("create failed");
        file.write_all(Department[i].as_bytes()).expect("create failed");
        file.write_all(" | ".as_bytes()).expect("create failed");
        file.write_all(Level[i].as_bytes()).expect("create failed");
        file.write_all("\n".as_bytes()).expect("create failed");
    }
    let mut file = std::fs::File::open("PAU_SIMS.txt").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    print!("{}", file_content );

}