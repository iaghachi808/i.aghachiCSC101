 fn main() {
 	
 	// program using formula for compound interest
 	
 	let p:f64 = 520_000_000; // principal is 520,000,000
 	let r:f64 = 10           // rate is 10%
 	let n:f64 = 5            // number of years is 5 years

 	let a:f64 = p * ( 1.0 + (r / 100.0)) * n;
 	println!("The Amount is {}", a);
 	let ci = a - p
 	println!("The compound is {}", ci);
 	
 }