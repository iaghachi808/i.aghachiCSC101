use std::io::Write;

fn main() {
    let Lager = vec!["LAGER","\n33 Export", "\nDesperasdos", "\nGoldberg", "\nGulder", "\nHeineken", "\nStar","\n"];
    let Stout = vec!["\n","STOUT","\nLegend", "\nTurbo King", "\nWilliams", "\n", "\n", "\n"];
    let NonAlcoholic = vec!["NonAlcoholic","\nMaltina", "\nAmstel Malta", "\nMalta Gold", "\nFayrouz", "\n", "\n"];

    let mut file = std::fs::File::create("Project1.txt").expect("Create failed");
    

    for i in 0..Lager.len() {
        
        file.write_all(Lager[i].as_bytes()).expect("write failed");
        println!("Data written to file.");
    
        file.write_all(Stout[i].as_bytes()).expect("write failed");
        println!("Data written to file.");
   
        file.write_all(NonAlcoholic[i].as_bytes()).expect("write failed");
        println!("Data written to file.");
    } 




}