use std::fs::File;
use std::io::Read;

fn main() {
    // Attempt to open the file
    let mut file = match File::open("staff_tb.sql") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            std::process::exit(1);
        }
    };

    // Read the contents of the file into a String
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            // Print the contents if read is successful
            print!("{}", contents);
        }
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            std::process::exit(1);
        }
    }
}








