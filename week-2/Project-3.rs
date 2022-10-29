 fn main() {
 	let p:f64 = 210_000.0;
 	let r:f64 = 5.0;
 	let n:f64 = 3.0;

 	let b:f64 = 1.0 - (r / 100.0);
 	let b = f64 :: powf(b,n);
    let a = p * b;
    println!("The Amount is {}", a);
 	let ci = a - p;
 	println!("The Depreciation is {}", ci);
 	
 }