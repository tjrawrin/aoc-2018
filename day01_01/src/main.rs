use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// Set the current frequency to a starting point of 0
	let mut current_frequency = 0;

	// Open the input file or display an error if the file failed to open
	let file = File::open("input.txt").expect("Unable to open the file!");
	// Create a vector of numbers based on each line in the opened input file
	let frequency_changes: Vec<i32> = BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().parse::<i32>().unwrap())
		.collect();

	// Loop over each frequency change
	for frequency_change in frequency_changes {
		// Calculate a new current frequency
		current_frequency += frequency_change;
	}
	// Print the final frequency Result
	println!("{}", current_frequency);
}
