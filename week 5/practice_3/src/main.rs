// Rust program to determined the area of a triangle with specified base and height

use std::io;

fn main() {
	let mut input_1 = String::new();
	let mut input_2 = String::new();

	println!("Enter Base");
	io::stdin().read_line(&mut input_1).expect("Not a valid String");
	let base:f32 = input_1.trim().parse().expect("Not a valid number");

	println!("Enter height");
	io::stdin().read_line(&mut input_2).expect("Not a valid String");
	let height:f32 = input_2.trim().parse().expect("Not a valid Number");

	if base > 0.0 {
		let area:f32 = (base * height) / 2.0;
		println!("Area of the triangle is {}",area);
	}

}
