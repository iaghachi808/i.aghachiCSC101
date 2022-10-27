   fn main() {
   	  let t:f64 = 450_000.00;   // amount for Toshiba
   	  let nt:f64 = 2.0;           // quantity of Toshiba laptops
   	  let m:f64 = 1_500_000.00; // amount for Mac
   	  let h:f64 = 750_000.00;   // amount for HP
   	  let nh:f64 = 3.0;           // quantity of HP laptops
   	  let d:f64 = 2_850_000.00;  // amount for Dell
   	  let nd:f64 = 3.0;           // quantity of Dell laptops
   	  let a:f64 = 250_000.0;      // amount for Acer 

   	  let total = (t * nt) + m + (h * nh) + (d * nd) + a;
   	  println!("The total is {}", total);

   	  let average = total / 5.0;
   	  println!("The average is {}", average);

 	
 }