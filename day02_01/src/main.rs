use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let mut total = HashMap::new();

	let file = File::open("input.txt").expect("Unable to open the file!");
	for line in BufReader::new(file).lines() {
		let mut count = HashMap::new();

		let line = line.unwrap();
		for character in line.bytes() {
			*count.entry(character).or_insert(0) += 1;
		}

		let mut seen_two = false;
		let mut seen_three = false;
		for (_key, val) in count.iter() {
			if val == &2 && !seen_two {
				seen_two = true;
				*total.entry("2s").or_insert(0) += 1;
			}
			if val == &3 && !seen_three {
				seen_three = true;
				*total.entry("3s").or_insert(0) += 1;
			}
		}
	}

	println!("{}", total["2s"] * total["3s"]);
}
