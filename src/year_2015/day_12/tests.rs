#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		sum_json_number,
		sum_json_number_without_red,
	};
	use serde_json::Value;
	
	#[test]
	fn test_sum_json_number_input() {
		let input: &String =
			&load_from_file("./src/year_2015/day_12/inputs.txt")[0];
		let data: Value = serde_json::from_str(input).unwrap();
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &data);
		assert_eq!(sum, 191164);
	}
	
	#[test]
	fn test_sum_json_number_without_red_input() {
		let input: &String =
			&load_from_file("./src/year_2015/day_12/inputs.txt")[0];
		let data: Value = serde_json::from_str(input).unwrap();
		let mut sum: i64 = 0;
		sum_json_number_without_red(&mut sum, &data);
		assert_eq!(sum, 87842);
	}
	
}