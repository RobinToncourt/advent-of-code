#[cfg(test)]
mod tests {
	use super::super::solutions::{
		look_and_say,
		get_result_length,
	};
	
	const INPUT: &str = "1321131112";
	
	#[test]
	fn test_get_result_length_examples() {
		assert_eq!(look_and_say("1"), "11");
		assert_eq!(look_and_say("11"), "21");
		assert_eq!(look_and_say("21"), "1211");
		assert_eq!(look_and_say("1211"), "111221");
		assert_eq!(look_and_say("111221"), "312211");
	}
	
	#[test]
	fn test_get_result_length_40_times() {
		assert_eq!(get_result_length(INPUT, 40), 492982);
	}
	
	#[test]
	fn test_get_result_length_50_times() {
		assert_eq!(get_result_length(INPUT, 50), 6989950);
	}
	
}