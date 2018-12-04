use day02_01::calculate_checksum;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// Get command line arguments
	let args: Vec<String> = env::args().collect();

	// Get the filename
	let filename = &args[1];

	// Open a file containing input values
	let file = File::open(filename).expect("Unable to open the file!");

	// Read each line and add it to a vector of Strings
	let ids: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

	// Print the result
	println!("{}", calculate_checksum(ids));
}
