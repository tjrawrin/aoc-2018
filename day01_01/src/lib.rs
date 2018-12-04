// Calculate the frequency change
pub fn find_frequency(frequency: i32, change: i32) -> i32 {
	frequency + change
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_finds_the_first_set_of_frequencies() {
		let mut result = 0;
		let changes = vec![1, 1, 1];
		for change in changes {
			result = find_frequency(result, change);
		}
		assert_eq!(result, 3);
	}

	#[test]
	fn it_finds_the_second_set_of_frequencies() {
		let mut result = 0;
		let changes = vec![1, 1, -2];
		for change in changes {
			result = find_frequency(result, change);
		}
		assert_eq!(result, 0);
	}

	#[test]
	fn it_finds_the_third_set_of_frequencies() {
		let mut result = 0;
		let changes = vec![-1, -2, -3];
		for change in changes {
			result = find_frequency(result, change);
		}
		assert_eq!(result, -6);
	}
}
