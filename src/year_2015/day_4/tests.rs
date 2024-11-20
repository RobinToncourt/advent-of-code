#[cfg(test)]
mod test {
	use super::super::solutions::{
		get_first_hash_start_five_zero,
		get_first_hash_start_six_zero
	};
	
	const SECRET_KEY: &str = "iwrupvqb";
	
	#[test]
	fn test_get_first_hash_start_five_zero_examples() {
		assert_eq!(
			get_first_hash_start_five_zero("abcdef").ok().unwrap(),
			609043
		);
		assert_eq!(
			get_first_hash_start_five_zero("pqrstuv").ok().unwrap(),
			1048970
		);
	}

	#[test]
	fn test_get_first_hash_start_five_zero_puzzle_input() {
		assert_eq!(
			get_first_hash_start_five_zero(SECRET_KEY).ok().unwrap(),
			346386
		);
	}

	#[test]
	fn test_get_first_hash_start_six_zero_puzzle_input() {
		assert_eq!(
			get_first_hash_start_six_zero(SECRET_KEY).ok().unwrap(),
			9958218
		);
	}
}
