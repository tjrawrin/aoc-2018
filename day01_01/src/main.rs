use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let mut count = 0;
	let file = File::open("input.txt").expect("Unable to open the file!");
	for line in BufReader::new(file).lines() {
		let num: i32 = line.unwrap().parse().unwrap();
		count += num;
		println!("{}", count);
	}
}
