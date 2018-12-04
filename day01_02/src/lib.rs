use std::collections::HashMap;

// Find the first double occurrence
pub fn find_first_double_occurrence(mut frequency: i32, changes: Vec<i32>) -> i32 {
	let mut seen_values = HashMap::new();
	seen_values.insert(frequency, 1);

	'outer: loop {
		for change in &changes {
			frequency += change;
			if seen_values.contains_key(&frequency) {
				break 'outer;
			}
			seen_values.insert(frequency, 1);
		}
	}

	frequency
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_finds_the_first_frequency_reached_twice_01() {
		let frequency_changes = vec![1, -1];
		let result = find_first_double_occurrence(0, frequency_changes);
		assert_eq!(result, 0);
	}

	#[test]
	fn it_finds_the_first_frequency_reached_twice_02() {
		let frequency_changes = vec![3, 3, 4, -2, -4];
		let result = find_first_double_occurrence(0, frequency_changes);
		assert_eq!(result, 10);
	}

	#[test]
	fn it_finds_the_first_frequency_reached_twice_03() {
		let frequency_changes = vec![-6, 3, 8, 5, -6];
		let result = find_first_double_occurrence(0, frequency_changes);
		assert_eq!(result, 5);
	}

	#[test]
	fn it_finds_the_first_frequency_reached_twice_04() {
		let frequency_changes = vec![7, 7, -2, -7, -4];
		let result = find_first_double_occurrence(0, frequency_changes);
		assert_eq!(result, 14);
	}
}
