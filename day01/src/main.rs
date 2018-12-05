use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];

	let content = fs::read_to_string(filename).expect("Unable to read file");

	let frequencies = content.split('\n').collect::<Vec<&str>>();
	let frequencies = frequencies
		.iter()
		.map(|f| f.trim().parse::<i32>())
		.collect::<Vec<Result<i32, _>>>();

	// Part 1
	let frequency = frequencies.iter().fold(0, |acc, n| match n {
		Ok(num) => acc + num,
		Err(err) => {
			println!("Error: {}", err);
			acc
		}
	});

	println!("Part 1:\t {}", frequency);

	// Part 2
	let mut seen_frequencies = HashMap::new();
	seen_frequencies.insert(0, 0);
	let mut acc = 0;

	'outer: loop {
		for f in frequencies.iter() {
			if let Ok(val) = f {
				acc += val;
				if seen_frequencies.contains_key(&acc) {
					break 'outer;
				}
				seen_frequencies.insert(acc, 0);
			}
		}
	}

	println!("Part 2:\t {}", acc);
}
