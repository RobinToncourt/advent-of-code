#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		get_shortest_route,
		get_longest_route,
	};
	
	#[test]
	fn test_get_shortest_route_example() {
		let inputs: Vec<String> = 
			load_from_file("./src/year_2015/day_9/example.txt");
		assert_eq!(get_shortest_route(&inputs).1, 605);
	}
	
	#[test]
	fn test_get_shortest_route_inputs() {
		let inputs: Vec<String> = 
			load_from_file("./src/year_2015/day_9/inputs.txt");
		assert_eq!(get_shortest_route(&inputs).1, 117);
	}
	
	#[test]
	fn test_get_longest_route_example() {
		let inputs: Vec<String> = 
			load_from_file("./src/year_2015/day_9/example.txt");
		assert_eq!(get_longest_route(&inputs).1, 982);
	}
	
	#[test]
	fn test_get_longest_route_inputs() {
		let inputs: Vec<String> = 
			load_from_file("./src/year_2015/day_9/inputs.txt");
		assert_eq!(get_longest_route(&inputs).1, 909);
	}
	
}