 fn main() {
 	
 	// program using formula for compound interest
 	
 	let p:f64 = 520_000_000.0; // principal is 520,000,000
 	let r:f64 = 10.0;           // rate is 10%
 	let n:f64 = 5.0;        // number of years is 5 years

 	let b = 1.0 + (r / 100.0);
    let b = f64 :: powf(b,n);
 	let a = p * b;
    println!("The Amount is {}", a);
 	let ci = a - p;
 	println!("The compound is {}", ci);
 	
    }