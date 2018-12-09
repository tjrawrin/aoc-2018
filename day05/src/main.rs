use std::env;
use std::fs;

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];
	let content = fs::read_to_string(filename).expect("Unable to read file");
	let polymer = content.chars().collect::<Vec<char>>();

	// let input = "dabAcCaCBAcCcaDA";
	// let input_chars = input.chars().collect::<Vec<char>>();

	// Part 1
	println!("{:?}", react(polymer));
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
