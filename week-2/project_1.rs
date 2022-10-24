fn main() {
	let p:f64 = 52000000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	// simple interest computation
 	let a = p * ( 1.0 + (r / 100.0)) * t;
	let ci = a - p;
	
	// prints the compound interest
	println!("Compound Interest is N{}", ci);

}