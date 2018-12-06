use std::collections::HashMap;
use std::env;
use std::fs;

type Claim = Vec<u32>;
type ClaimArea = (u32, Vec<(u32, u32)>);

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let filename = &args[1];
	let content = fs::read_to_string(filename).expect("Unable to read file");
	let lines = content.split('\n').collect::<Vec<&str>>();
	let claims = parse_claims(lines);
	let areas = get_claim_area(claims);
	let map = create_claim_map(&areas);

	// Part 1
	println!("{:?}", area_claimed_by_two_or_more(&map));

	// Part 2
	println!("{:?}", claim_that_does_not_overlap(&areas, &map));
}

fn parse_claims(v: Vec<&str>) -> Vec<Claim> {
	v.iter()
		.map(|s| {
			s.split(|c| c == ' ' || c == '#' || c == ',' || c == ':' || c == 'x')
				.filter_map(|c| c.parse::<u32>().ok())
				.collect::<Claim>()
		})
		.collect()
}
#[test]
fn parse_claims_test() {
	let input = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
	let output = vec![[1, 1, 3, 4, 4], [2, 3, 1, 4, 4], [3, 5, 5, 2, 2]];
	assert_eq!(parse_claims(input), output);
}

fn get_claim_area(v: Vec<Claim>) -> Vec<ClaimArea> {
	v.iter()
		.map(|c| {
			let mut coords = vec![];
			for dx in 0..c[3] {
				for dy in 0..c[4] {
					coords.push((c[1] + dx, c[2] + dy))
				}
			}
			(c[0], coords)
		})
		.collect()
}
#[test]
fn get_claim_area_test() {
	let input = vec![vec![3, 5, 5, 2, 2]];
	let output = vec![(3, vec![(5, 5), (5, 6), (6, 5), (6, 6)])];
	assert_eq!(get_claim_area(input), output);
}

fn create_claim_map(v: &[ClaimArea]) -> HashMap<&(u32, u32), u32> {
	let mut map = HashMap::new();
	v.iter().for_each(|(_id, ca)| {
		ca.iter().for_each(|c| {
			*map.entry(c).or_insert(0) += 1;
		})
	});
	map
}

fn area_claimed_by_two_or_more(m: &HashMap<&(u32, u32), u32>) -> u32 {
	let mut total = 0;
	for v in m {
		if *v.1 > 1 {
			total += 1;
		}
	}
	total
}

fn claim_that_does_not_overlap(v: &[(u32, Vec<(u32, u32)>)], m: &HashMap<&(u32, u32), u32>) -> u32 {
	let mut overlap_id = 0;
	v.iter().for_each(|(id, coords)| {
		let mut overlap = 0;
		coords.iter().for_each(|coord| {
			if m.get(coord) > Some(&1) {
				overlap += 1;
			}
		});
		if overlap == 0 {
			overlap_id = *id;
		}
	});
	overlap_id
}
