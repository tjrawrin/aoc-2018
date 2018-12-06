use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];

	let content = fs::read_to_string(filename).expect("Unable to read file");

	let lines = content.split('\n').collect::<Vec<&str>>();

	let claims = parse_claims(lines);

	let claim_map = generate_claim_map(&claims);

	// Part 1
	println!("{:?}", area_claimed_by_two_or_more(&claim_map));

	// Part 2
	println!("{:?}", claim_that_does_not_overlap(&claim_map, &claims));
}

fn parse_claims(ls: Vec<&str>) -> Vec<Vec<u32>> {
	ls.iter()
		.map(|l| {
			l.split(|c| c == ' ' || c == '#' || c == ',' || c == ':' || c == 'x')
				.filter_map(|c| c.parse::<u32>().ok())
				.collect::<(Vec<u32>)>()
		})
		.collect()
}

fn generate_claim_map(cs: &[Vec<u32>]) -> HashMap<(u32, u32), u32> {
	let mut map = HashMap::new();

	for c in cs {
		for dx in 0..c[3] {
			for dy in 0..c[4] {
				*map.entry((c[1] + dx, c[2] + dy)).or_insert(0) += 1;
			}
		}
	}

	// for line in lines {
	// 	let claim = line
	// 		.split(|c| c == ' ' || c == '#' || c == ',' || c == ':' || c == 'x')
	// 		.filter_map(|c| c.parse::<u32>().ok())
	// 		.collect::<Vec<u32>>();

	// 	for dx in 0..claim[3] {
	// 		for dy in 0..claim[4] {
	// 			*map.entry((claim[1] + dx, claim[2] + dy)).or_insert(0) += 1;
	// 		}
	// 	}
	// }

	map
}

fn area_claimed_by_two_or_more(m: &HashMap<(u32, u32), u32>) -> u32 {
	let mut total = 0;
	for v in m {
		if *v.1 > 1 {
			total += 1;
		}
	}
	total
}

fn claim_that_does_not_overlap(m: &HashMap<(u32, u32), u32>, cs: &[Vec<u32>]) -> Option<u32> {
	let mut ok = true;

	println!("{:?}", m);

	for c in cs {
		for dx in 0..c[3] {
			for dy in 0..c[4] {
				if m.get(&(c[1] + dx, c[2] + dy)) > Some(&1) {
					ok = false;
				}
			}
		}
		if ok {
			return Some(c[0]);
		}
	}

	None
}

// fn claim_that_does_not_overlap(m: &HashMap<(u32, u32), u32>) -> u32 {
// 	let mut ok = true;

// 	for (_k, v) in m {
// 		if v > &1 {
// 			ok = false;
// 		}
// 	}

// 	for line in lines {
// 		let claim = line
// 			.split(|c| c == ' ' || c == '#' || c == ',' || c == ':' || c == 'x')
// 			.filter_map(|c| c.parse::<u32>().ok())
// 			.collect::<Vec<u32>>();

// 		println!("{:?}", claim);

// 		for dx in 0..claim[3] {
// 			for dy in 0..claim[4] {
// 				*map.entry((claim[1] + dx, claim[2] + dy)).or_insert(0) += 1;
// 				if map.get(&(claim[1] + dx, claim[2] + dy)) > Some(&1) {
// 					ok = false;
// 				};
// 			}
// 		}

// 		if ok {
// 			return claim[0];
// 		}
// 	}

// 	3
// }
