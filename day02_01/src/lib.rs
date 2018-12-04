use std::collections::HashMap;

pub fn calculate_checksum(ids: Vec<String>) -> u32 {
	let mut total: HashMap<&str, u32> = HashMap::new();

	for id in ids {
		let mut count: HashMap<u8, u32> = HashMap::new();

		for c in id.bytes() {
			*count.entry(c).or_insert(0) += 1;
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

	total["2s"] * total["3s"]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_calculates_the_checksum_01() {
		let ids = vec![
			"abcdef".to_string(),
			"bababc".into(),
			"abbcde".into(),
			"abcccd".into(),
			"aabcdd".into(),
			"abcdee".into(),
			"ababab".into(),
		];
		assert_eq!(calculate_checksum(ids), 12);
	}
}
