use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Nap {
	guard_id: u32,
	sleep: u32,
	wake: u32,
}

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];
	let content = fs::read_to_string(filename).expect("Unable to read file");
	let mut entries = content.split('\n').collect::<Vec<&str>>();

	entries.sort();

	let mut guard_id = 0;
	let mut sleep = 0;
	let mut wake = 0;
	let mut naps = vec![];

	for entry in entries {
		let (_hour, minute) = parse_time(entry);

		if entry.contains("begins shift") {
			guard_id = parse_guard(entry);
		}
		if entry.contains("falls asleep") {
			sleep = minute;
		}
		if entry.contains("wakes up") {
			wake = minute;
			naps.push(Nap {
				guard_id,
				sleep,
				wake,
			});
		}
	}

	// Part 1
	let sleepiest_guard = sleepiest_guard(&naps);
	let sleepiest_minute = sleepiest_minute(&naps, sleepiest_guard);
	println!("{}", sleepiest_guard * sleepiest_minute);

	// Part 2
	let (guard_id, minute) = most_frequenty_asleep_minute(&naps);
	println!("{}", guard_id * minute);
}

fn parse_time(s: &str) -> (u32, u32) {
	let i = s
		.split(|c| c == ' ' || c == ':' || c == ']')
		.filter_map(|c| c.parse::<u32>().ok())
		.collect::<Vec<u32>>();
	(i[0], i[1])
}

fn parse_guard(s: &str) -> u32 {
	s.split(|c| c == ' ' || c == '#')
		.filter_map(|c| c.parse::<u32>().ok())
		.collect::<Vec<u32>>()[0]
}

fn sleepiest_guard(naps: &[Nap]) -> u32 {
	let mut sleep_counts = HashMap::new();
	for nap in naps {
		*sleep_counts.entry(nap.guard_id).or_insert(0) += nap.wake - nap.sleep;
	}
	let (id, _mins) = sleep_counts
		.iter()
		.max_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
		.unwrap_or((&0, &0));
	id.to_owned()
}

fn sleepiest_minute(naps: &[Nap], guard_id: u32) -> u32 {
	let mut minutes = HashMap::new();
	for nap in naps {
		if nap.guard_id == guard_id {
			for minute in nap.sleep..nap.wake {
				*minutes.entry(minute).or_insert(0) += 1;
			}
		}
	}
	let (min, _count) = minutes
		.iter()
		.max_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
		.unwrap_or((&0, &0));
	min.to_owned()
}

fn most_frequenty_asleep_minute(naps: &[Nap]) -> (u32, u32) {
	println!("{:?}", naps);
	let mut minutes = HashMap::new();
	for nap in naps {
		for minute in nap.sleep..nap.wake {
			*minutes.entry((nap.guard_id, minute)).or_insert(0) += 1;
		}
	}
	let ((id, min), _count) = minutes
		.iter()
		.max_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
		.unwrap_or((&(0, 0), &0));
	(id.to_owned(), min.to_owned())
}
