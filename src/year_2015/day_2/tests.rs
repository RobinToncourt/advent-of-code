#[cfg(test)]
pub mod tests {
    use crate::utils::utils::load_from_file;
    use super::super::solutions::{
		utils::inputs_to_presents,
		Present,
		wrapping_paper_order,
		ribbon_order
	};
    
    const EXAMPLE_1: Present = Present::new(2, 3, 4);
    const EXAMPLE_2: Present = Present::new(1, 1, 10);

    #[test]
    fn test_calculate_surface_area_examples() {
        assert_eq!(EXAMPLE_1.calculate_surface_area(), 52);
        assert_eq!(EXAMPLE_2.calculate_surface_area(), 42);
    }
    
    #[test]
    fn test_get_smallest_side_area_examples() {
        assert_eq!(EXAMPLE_1.get_smallest_side_area(), 6);
        assert_eq!(EXAMPLE_2.get_smallest_side_area(), 1);
    }
    
    #[test]
    fn test_get_wrapping_paper_needed_examples() {
        assert_eq!(EXAMPLE_1.get_wrapping_paper_needed(), 58);
        assert_eq!(EXAMPLE_2.get_wrapping_paper_needed(), 43);
    }
    
    #[test]
    fn test_wrapping_paper_order_examples() {
        assert_eq!(wrapping_paper_order(vec![EXAMPLE_1, EXAMPLE_2]), 101);
    }
    
    #[test]
    fn test_wrapping_paper_order_puzzle_inputs() {
        let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_2/inputs.txt");
        let presents: Vec<Present> = inputs_to_presents(inputs);
        assert_eq!(wrapping_paper_order(presents), 1588178);
    }
    
    #[test]
    fn test_get_ribbon_needed_examples() {
        assert_eq!(EXAMPLE_1.get_ribbon_needed(), 34);
        assert_eq!(EXAMPLE_2.get_ribbon_needed(), 14);
    }
    
    #[test]
    fn test_get_ribbon_needed_puzzle_inputs() {
        let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_2/inputs.txt");
        let presents: Vec<Present> = inputs_to_presents(inputs);
        assert_eq!(ribbon_order(presents), 3783758);
    }
}