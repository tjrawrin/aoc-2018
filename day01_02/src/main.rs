use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	// Set the current frequency to a starting point of 0
	let mut current_frequency = 0;
	// Create a new hash map to store the calculated frequencies
	let mut frequency_list = HashMap::new();
	// Insert the starting "current frequency" into the hash map
	frequency_list.insert(current_frequency.to_string(), 1);

	// Open the input file or display an error if the file failed to open
	let file = File::open("input.txt").expect("Unable to open the file!");
	// Create a vector of numbers based on each line in the opened input file
	let frequency_changes: Vec<i32> = BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().parse::<i32>().unwrap())
		.collect();

	// Keep looping until we come across a duplicate key in the hash map
	'outer: loop {
		// Loop over each frequency change
		for frequency_change in &frequency_changes {
			// Calculate a new current frequency
			current_frequency += frequency_change;
			// If the hash map contains a frequency we have already seen. Print it and break out of the loop
			if frequency_list.contains_key(&current_frequency.to_string()) {
				println!("{}", current_frequency);
				break 'outer;
			}
			// Insert the new calculated frequency into the hash map
			frequency_list.insert(current_frequency.to_string(), 1);
		}
	}
}
