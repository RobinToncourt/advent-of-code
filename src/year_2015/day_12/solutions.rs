#![allow(dead_code)]
pub fn sum_json_number(sum: &mut i64, data: &serde_json::Value) {
	match data {
		serde_json::Value::Null => {},
		serde_json::Value::Bool(_) => {},
		serde_json::Value::Number(number) => {
			if let Some(value) = number.as_i64() {
				*sum += value;
			}
		},
		serde_json::Value::String(_) => {},
		serde_json::Value::Array(array) => {
			for i in 0..array.len() {
				sum_json_number(sum, &array[i]);
			}
		},
		serde_json::Value::Object(dict) => {
			for value in dict.values() {
				sum_json_number(sum, &value);
			}
		},
	}
}

pub fn sum_json_number_without_red(sum: &mut i64, data: &serde_json::Value) {
	match data {
		serde_json::Value::Null => {},
		serde_json::Value::Bool(_) => {},
		serde_json::Value::Number(number) => {
			if let Some(value) = number.as_i64() {
				*sum += value;
			}
		},
		serde_json::Value::String(_) => {},
		serde_json::Value::Array(array) => {
			for i in 0..array.len() {
				sum_json_number_without_red(sum, &array[i]);
			}
		},
		serde_json::Value::Object(dict) => {
			let mut ignore: bool = false;

			for value in dict.values() {
				if value == "red" {
					ignore = true;
				}
			}
			
			if !ignore {
				for value in dict.values() {
					sum_json_number_without_red(sum, &value);
				}
			}
		},
	}
}