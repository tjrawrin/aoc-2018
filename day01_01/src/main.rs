use day01_01::find_frequency;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
	// Current frequency
	let mut frequency = 0;

	// Get command line arguments
	let args: Vec<String> = env::args().collect();

	// Get the filename
	let filename = &args[1];

	// Open a file containing input values
	let file = File::open(filename).expect("Unable to open the file!");

	// Read each line and calculate the new frequency
	for line in BufReader::new(file).lines() {
		let line = line.unwrap().parse::<i32>().unwrap();
		frequency = find_frequency(frequency, line);
	}

	// Print the result
	println!("{}", frequency);
}
