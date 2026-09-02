fn main () {
	let t: f64 = 450_000.0;
	let m: f64 = 1_500_000.00;
	let h: f64 = 750_000.00;
	let d: f64 = 2_850_000.00;
	let a: f64 = 250_000.00;

	// sum 
	let s = (t * 2.0) + (m * 1.0) + (h * 3.0) + (d * 3.0) + (a * 1.0);
	println!("Sum is {}", s);

	// average
	let av = s / 10.0;
	println!("Average Sum is {}", av);


}