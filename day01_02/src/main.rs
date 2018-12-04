use day01_02::find_first_double_occurrence;
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

	// Read each line and add it to a vector of numbers
	let frequency_changes: Vec<i32> = BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().parse::<i32>().unwrap())
		.collect();

	// Print the result
	println!("{}", find_first_double_occurrence(0, frequency_changes));
}
