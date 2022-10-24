fn main() {

	// Imputs the values for the computation
	let q1:f64 = 2.0;
	let q2:f64 = 1.0;
	let q3:f64 = 3.0;
	let q4:f64 = 3.0;
	let q5:f64 = 1.0;

	let a1:f64 = 450000.0;
	let a2:f64 = 1500000.0;
	let a3:f64 = 750000.0;
	let a4:f64 = 2850000.0;
	let a5:f64 = 250000.0;

	let t1:f64 = q1 * a1;
	let t2:f64 = q2 * a2;
	let t3:f64 = q3 * a3;
	let t4:f64 = q4 * a4;
	let t5:f64 = q5 * a5;

	// Computation of the sum of the amount in sales
	let sum:f64 = t1+t2+t3+t4+t5;

	// Prints the the sum of the amount in sales
	println!("The total amount of sales is N{}", sum);

}