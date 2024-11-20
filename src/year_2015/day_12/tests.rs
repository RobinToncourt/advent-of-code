#[cfg(test)]
mod tests {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		sum_json_number,
		sum_json_number_without_red,
	};
	use serde_json::Value;
	
	const EXAMPLE_1: &str = "[1,2,3]";
	const EXAMPLE_2: &str = r#"{"a":2,"b":4}"#;
	
	const EXAMPLE_3: &str = r#"[[[3]]]"#;
	const EXAMPLE_4: &str = r#"{"a":{"b":4},"c":-1}"#;
	
	const EXAMPLE_5: &str = r#"{"a":[-1,1]}"#;
	const EXAMPLE_6: &str = r#"[-1,{"a":1}]"#;
	
	const EXAMPLE_7: &str = r#"[]"#;
	const EXAMPLE_8: &str = r#"{}"#;
	
	#[test]
	fn test_sum_json_number_examples() {
		let ex_1: Value = serde_json::from_str(EXAMPLE_1).unwrap();
		let ex_2: Value = serde_json::from_str(EXAMPLE_2).unwrap();
		
		let ex_3: Value = serde_json::from_str(EXAMPLE_3).unwrap();
		let ex_4: Value = serde_json::from_str(EXAMPLE_4).unwrap();
		
		let ex_5: Value = serde_json::from_str(EXAMPLE_5).unwrap();
		let ex_6: Value = serde_json::from_str(EXAMPLE_6).unwrap();
		
		let ex_7: Value = serde_json::from_str(EXAMPLE_7).unwrap();
		let ex_8: Value = serde_json::from_str(EXAMPLE_8).unwrap();
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_1);
		assert_eq!(sum, 6);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_2);
		assert_eq!(sum, 6);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_3);
		assert_eq!(sum, 3);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_4);
		assert_eq!(sum, 3);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_5);
		assert_eq!(sum, 0);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_6);
		assert_eq!(sum, 0);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_7);
		assert_eq!(sum, 0);
		
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &ex_8);
		assert_eq!(sum, 0);
	}
	
	#[test]
	fn test_sum_json_number_input() {
		let input: &String =
			&load_from_file("./src/year_2015/day_12/inputs.txt")[0];
		let data: Value = serde_json::from_str(input).unwrap();
		let mut sum: i64 = 0;
		sum_json_number(&mut sum, &data);
		assert_eq!(sum, 191164);
	}
	
	const EXAMPLE_9: &str = r#"[1,{"c":"red","b":2},3]"#;
	const EXAMPLE_10: &str = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
	const EXAMPLE_11: &str = r#"[1,"red",5]"#;
	
	#[test]
	fn test_sum_json_number_without_red_examples() {
		let ex_1: Value = serde_json::from_str(EXAMPLE_1).unwrap();
		let ex_9: Value = serde_json::from_str(EXAMPLE_9).unwrap();
		
		let ex_10: Value = serde_json::from_str(EXAMPLE_10).unwrap();
		let ex_11: Value = serde_json::from_str(EXAMPLE_11).unwrap();
		
		let mut sum: i64 = 0;
		sum_json_number_without_red(&mut sum, &ex_1);
		assert_eq!(sum, 6);
		
		let mut sum: i64 = 0;
		sum_json_number_without_red(&mut sum, &ex_9);
		assert_eq!(sum, 4);
		
		let mut sum: i64 = 0;
		sum_json_number_without_red(&mut sum, &ex_10);
		assert_eq!(sum, 0);
		
		let mut sum: i64 = 0;
		sum_json_number_without_red(&mut sum, &ex_11);
		assert_eq!(sum, 6);
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