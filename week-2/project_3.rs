fn main () {
	// Accepting values to be computed
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let _n:f64 = 3.0;

	// Computes depreciation
	let b:f64 = ( 1.0 - ( r / 100.0 )).powf(3.0);
	let a:f64 = p * b;

	// Prints the value of the TV after depreciation
	println!("The value of the TV after 3 years is N{}", a);
}