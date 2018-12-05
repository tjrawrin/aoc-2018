use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];

	let content = fs::read_to_string(filename).expect("Unable to read file");

	let box_ids = content.split('\n').collect::<Vec<&str>>();

	// Part 1
	println!("Part 1:\t {}", calculate_checksum(box_ids.clone()));

	// Part 2
	println!(
		"Part 2:\t {}",
		get_common_characters(off_by_one_character(box_ids))
	);
}

fn calculate_checksum(ids: Vec<&str>) -> u32 {
	let total_count = ids
		.iter()
		.map(|id| id.chars())
		.map(|chars| {
			let mut count = HashMap::new();
			chars.for_each(|c| *count.entry(c).or_insert(0) += 1);
			count
		})
		.map(|char_map| {
			let twos = char_map.iter().any(|(_k, v)| v == &2);
			let threes = char_map.iter().any(|(_k, v)| v == &3);
			(twos, threes)
		})
		.fold((0, 0), |mut acc, n| match n {
			(true, true) => {
				acc.0 += 1;
				acc.1 += 1;
				acc
			}
			(true, false) => {
				acc.0 += 1;
				acc
			}
			(false, true) => {
				acc.1 += 1;
				acc
			}
			_ => acc,
		});

	total_count.0 * total_count.1
}

#[test]
fn calculate_checksum_test() {
	let data = vec![
		"abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
	];
	assert_eq!(calculate_checksum(data), 12);
}

fn off_by_one_character(ids: Vec<&str>) -> Vec<&str> {
	let mut new_box_ids = vec![];

	for i in ids.iter() {
		for j in ids.iter() {
			let mut i_chars = i.chars();
			let mut j_chars = j.chars();
			let mut count = 0;

			loop {
				let i_char = i_chars.next();
				let j_char = j_chars.next();
				if i_char == None {
					break;
				}
				if i_char != j_char {
					count += 1;
				}
			}
			if count == 1 {
				new_box_ids.push(*j);
			}
		}
	}

	new_box_ids
}

#[test]
fn off_by_one_character_test() {
	let data = vec![
		"abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
	];
	assert_eq!(off_by_one_character(data), vec!["fguij", "fghij"]);
}

fn get_common_characters(ids: Vec<&str>) -> String {
	let mut result = ids[0].chars().collect::<Vec<char>>();

	for (i, v) in ids[1].chars().enumerate() {
		if v != result[i] {
			result.remove(i);
			break;
		}
	}

	result.iter().collect::<String>()
}

#[test]
fn get_common_characters_test() {
	assert_eq!(get_common_characters(vec!["fguij", "fghij"]), "fgij");
}
