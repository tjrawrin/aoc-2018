use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];
	let content = fs::read_to_string(filename).expect("Unable to read file");
	let mut polymer = content.chars().collect::<Vec<char>>();

	// Part 1
	// println!("{:?}", react(polymer));

	// Part 2
	println!("{:?}", find_problem_unit(polymer));
}

fn react(mut polymer: Vec<char>) -> usize {
	let mut start = 0;
	loop {
		if start + 1 < polymer.len() {
			let current = polymer[start];
			let next = polymer[start + 1];
			let current_char = current.to_string().to_lowercase();
			let next_char = next.to_string().to_lowercase();
			if current_char == next_char && current != next {
				polymer.remove(start + 1);
				polymer.remove(start);
				if start != 0 {
					start -= 1;
				}
			} else {
				start += 1;
			}
		} else {
			break;
		}
	}
	polymer.len()
}
#[test]
fn react_test() {
	let polymer = "dabAcCaCBAcCcaDA".chars().collect::<Vec<char>>();
	assert_eq!(react(polymer), 10);
}

fn find_problem_unit(polymer: Vec<char>) -> Option<(String, usize)> {
	let mut units = polymer
		.clone()
		.iter()
		.map(|c| c.to_lowercase().to_string())
		.collect::<Vec<String>>();
	units.sort();
	units.dedup();
	let mut counter = HashMap::new();
	units.iter().for_each(|unit| {
		let mut polymer = polymer.clone();
		polymer.retain(|&c| {
			&c.to_string() != unit && c.to_string() != *unit.to_uppercase().to_string()
		});
		let count = react(polymer);
		counter.insert(unit.to_string(), count);
	});
	counter
		.into_iter()
		.min_by(move |(_k1, v1), (_k2, v2)| v1.cmp(v2))
}
#[test]
fn find_problem_unit_test() {
	let polymer = "dabAcCaCBAcCcaDA".chars().collect::<Vec<char>>();
	assert_eq!(find_problem_unit(polymer), Some(("c".to_string(), 4)));
}
