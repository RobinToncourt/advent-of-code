#[cfg(test)]
pub mod tests {
    use crate::utils::utils::load_from_file;
    use super::super::solutions::{
		at_least_one_present,
		at_least_one_present_with_robo_santa
	};
    
    #[test]
    fn test_at_least_one_present_examples() {
        assert_eq!(at_least_one_present(">"), 2);
        assert_eq!(at_least_one_present("^>v<"), 4);
        assert_eq!(at_least_one_present("^v^v^v^v^v"), 2);
    }
    
    #[test]
    fn test_at_least_one_present_puzzle_inputs() {
        let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_3/inputs.txt");
        assert_eq!(at_least_one_present(inputs.get(0).unwrap()), 2565);
    }
    
    #[test]
    fn test_at_least_one_present_with_robo_santa_examples() {
        assert_eq!(at_least_one_present_with_robo_santa("^v"), 3);
        assert_eq!(at_least_one_present_with_robo_santa("^>v<"), 3);
        assert_eq!(at_least_one_present_with_robo_santa("^v^v^v^v^v"), 11);
    }
    
    #[test]
    fn test_at_least_one_present_with_robo_santa_puzzle_inputs() {
        let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_3/inputs.txt");
        assert_eq!(
			at_least_one_present_with_robo_santa(inputs.get(0).unwrap()),
			2639
		);
    }
}