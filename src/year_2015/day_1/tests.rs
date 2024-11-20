#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{which_floor, position_enter_basement};
	
	#[test]
	fn test_which_floor_examples() {
		assert_eq!(which_floor("(())"), 0);
		assert_eq!(which_floor("()()"), 0);
		
		assert_eq!(which_floor("((("), 3);
		assert_eq!(which_floor("(()(()("), 3);
		assert_eq!(which_floor("))((((("), 3);
		
		assert_eq!(which_floor("())"), -1);
		assert_eq!(which_floor("))("), -1);
		
		assert_eq!(which_floor(")))"), -3);
		assert_eq!(which_floor(")())())"), -3);
	}

	#[test]
	fn test_which_floor_puzzle_inputs() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_1/inputs.txt");
		assert_eq!(which_floor(inputs.get(0).unwrap().as_str()), 280);
	}
	
	#[test]
	fn test_position_enter_basement_examples() {
		assert_eq!(position_enter_basement(")"), 1);
		assert_eq!(position_enter_basement("()())"), 5);
	}

	#[test]
	fn test_position_enter_basement_puzzle_inputs() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_1/inputs.txt");
		assert_eq!(
			position_enter_basement(inputs.get(0).unwrap().as_str()),
			1797
		);
	}
}