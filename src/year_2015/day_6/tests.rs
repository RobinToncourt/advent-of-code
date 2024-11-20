#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		get_how_many_light_lit,
		get_total_brightness
	};
	
	#[test]
	fn test_get_how_many_light_lit_inputs() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_6/inputs.txt");
		assert_eq!(get_how_many_light_lit(inputs), 377891);
	}
	
	#[test]
	fn test_get_total_brightness_examples() {
		let inputs: Vec<String> =
			vec!["turn on 0,0 through 0,0".to_string()];
		assert_eq!(get_total_brightness(inputs), 1);
		
		let inputs: Vec<String> =
			vec!["toggle 0,0 through 999,999".to_string()];
		assert_eq!(get_total_brightness(inputs), 2_000_000);
	}
	
	#[test]
	fn test_get_total_brightness_inputs() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_6/inputs.txt");
		assert_eq!(get_total_brightness(inputs), 14110788);
	}
	
}