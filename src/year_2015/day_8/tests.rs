#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		get_decode_string_diff_sum,
		get_encode_string_diff_sum,
	};
	
	#[test]
	fn test_get_decode_string_diff_sum_example() {
		let example: Vec<String> = load_from_file("./src/year_2015/day_8/example.txt");
		assert_eq!(get_decode_string_diff_sum(example), 12);
	}
	
	#[test]
	fn test_get_decode_string_diff_sum_inputs() {
		let inputs: Vec<String> = load_from_file("./src/year_2015/day_8/inputs.txt");
		assert_eq!(get_decode_string_diff_sum(inputs), 1342);
	}
	
	#[test]
	fn test_get_encode_string_diff_sum_example() {
		let example: Vec<String> = load_from_file("./src/year_2015/day_8/example.txt");
		assert_eq!(get_encode_string_diff_sum(example), 19);
	}
	
	#[test]
	fn test_get_encode_string_diff_sum_inputs() {
		let inputs: Vec<String> = load_from_file("./src/year_2015/day_8/inputs.txt");
		assert_eq!(get_encode_string_diff_sum(inputs), 2074);
	}
	
}
