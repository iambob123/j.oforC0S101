fn main(){
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// Calculation of depreciation
	let z:f64 = p * (1.0 -(( r / 100.0 )* n));
	println!("The amount Is {}", z);

} 

