#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		resolve_puzzle_one,
		resolve_puzzle_two,
	};
	
	#[test]
	fn test_resolve_puzzle_one_example() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_13/example.txt");
		assert_eq!(resolve_puzzle_one(inputs), 330);
	}
	
	#[test]
	fn test_resolve_puzzle_one_inputs() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_13/inputs.txt");
		assert_eq!(resolve_puzzle_one(inputs), 664);
	}
	
	#[test]
	fn test_resolve_puzzle_two_inputs() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_13/inputs.txt");
		assert_eq!(resolve_puzzle_two(inputs), 640);
	}
	
}