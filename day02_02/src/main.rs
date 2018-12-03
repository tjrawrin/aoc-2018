use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let file = File::open("input.txt").expect("Unable to open the file!");

	let box_ids = BufReader::new(file)
		.lines()
		.map(|l| l.unwrap())
		.collect::<Vec<String>>();

	'outer: for outer_box_id in &box_ids {
		println!("{:?}", outer_box_id);
		for inner_box_id in &box_ids {
			println!("\t{:?}", inner_box_id);
			let differences = count_differences(outer_box_id, inner_box_id);
			if differences == 1 {
				println!("{} {}", outer_box_id, inner_box_id);
				println!("{}", print_common_characters(outer_box_id, inner_box_id));
				break 'outer;
			}
		}
	}
}

fn count_differences(string_one: &str, string_two: &str) -> i32 {
	let mut string_one_characters = string_one.chars();
	let mut string_two_characters = string_two.chars();
	let mut count = 0;
	loop {
		let string_one_character = string_one_characters.next();
		let string_two_character = string_two_characters.next();
		if string_one_character == None {
			break;
		}
		if string_one_character != string_two_character {
			count += 1;
		}
	}
	count
}

fn print_common_characters(string_one: &str, string_two: &str) -> String {
	let mut string_one_characters = string_one.chars();
	let mut string_two_characters = string_two.chars();
	let mut result = String::new();
	loop {
		let string_one_character = string_one_characters.next();
		let string_two_character = string_two_characters.next();
		if string_one_character == None {
			break;
		}
		if string_one_character == string_two_character {
			result.push(string_one_character.unwrap());
		}
	}
	result
}
