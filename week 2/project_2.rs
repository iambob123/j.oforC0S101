fn main(){
	let toshiba:f64 = 450_000_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	// Sum total
	let k:f64 = toshiba + mac + hp + dell + acer;
	println!("The sum total is {}", k);

	// Average
	let z:f64 = k / 5.0;
	println!("The Average is {}", z);

}
