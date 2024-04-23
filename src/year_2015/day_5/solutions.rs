#![allow(dead_code)]

// Part 1.

use std::collections::HashMap;
use regex::Regex;

const REGEX_VOWELS: &str = "[^aeiouAEIOU]";
const FORBIDDEN_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn contains_3_vowels(string: &str) -> bool {
	Regex::new(REGEX_VOWELS).unwrap().replace_all(string, "").len() >= 3
}

pub fn contains_characters_twice_in_a_row(string: &str) -> bool {
	for i in 0..string.len()-1 {
		let char_n0: char = string.chars().nth(i).unwrap();
		let char_n1: char = string.chars().nth(i+1).unwrap();

		if char_n0 == char_n1 {
			return true;
		}
	}

	false
}

pub fn contains_forbidden_string(string: &str) -> bool {
	for fbdn_str in FORBIDDEN_STRINGS {
		if string.contains(fbdn_str) {
			return true;
	   }
	}

	false
}

pub fn is_nice_string(string: &str) -> bool {
	contains_3_vowels(string) &
	contains_characters_twice_in_a_row(string) &
	& !contains_forbidden_string(string)
}

pub fn how_many_nice_strings(inputs: Vec<String>) -> u16 {
	let mut nice_string_counter: u16 = 0;

	for string in inputs {
		if is_nice_string(&string) {
			nice_string_counter += 1;
		}
	}

	nice_string_counter
}

// Part 2.

// Ne fonctionne pas.
// Si la pair est du type 'sknufchjdvccccta' => 'cccc'
// la condition est à changer.
pub fn contains_pair_without_overlapping(string: &str) -> bool {
	let mut pair_map: HashMap<String, u8> = HashMap::new();
	let mut previous_pair: String = String::from("");

	for i in 0..string.len()-1 {
		let char_n0: char = string.chars().nth(i).unwrap();
		let char_n1: char = string.chars().nth(i+1).unwrap();

		let mut pair: String = String::from(char_n0);
		pair.push(char_n1);

		if pair != previous_pair {
			let count = match pair_map.get(&pair) {
				Some(count) => count+1,
				None => 1,
			};

			pair_map.insert(pair.clone(), count);
		}

		previous_pair = pair;
	}

	for (_key, value) in pair_map.into_iter() {
		if value >= 2 {
			return true;
		}
	}  

	false
}

// Ne fonctionne pas.
// Si la pair est du type 'sknufchjdvccccta' => 'cccc'
// la condition est à changer.
pub fn contains_pair_without_overlapping_2(string: &str) -> bool {
	let mut list: Vec<String> = Vec::new();
	let mut previous_pair: String = String::from("");

	for i in 0..string.len()-1 {
		let char_n0: char = string.chars().nth(i).unwrap();
		let char_n1: char = string.chars().nth(i+1).unwrap();

		let mut pair: String = String::from(char_n0);
		pair.push(char_n1);

		if pair != previous_pair {
			if list.contains(&pair) {
				return true;
			}
			else {
				list.push(pair.clone());
			}
		}

		previous_pair = pair;
	}

	false
}

pub fn contains_pair_without_overlapping_3(string: &str) -> bool {
	let mut i: usize = 1;

	while i < string.len() {
		let char_n0: char = string.chars().nth(i-1).unwrap();
		let char_n1: char = string.chars().nth(i).unwrap();

		let mut pair: String = String::from(char_n0);
		pair.push(char_n1);

		if string[i+1..].contains(&pair) {
			return true;
		}

		i += 1
	}

	false
}

pub fn contains_letters_repeated_separated_by_one_letter
(string: &str) -> bool {
	for i in 0..string.len()-2 {
		let char_n0: char = string.chars().nth(i).unwrap();
		let char_n2: char = string.chars().nth(i+2).unwrap();

		if char_n0 == char_n2 {
			return true;
		}
	}

	false
}

pub fn is_nice_string_better_model(string: &str) -> bool {
	contains_pair_without_overlapping_3(string) &
	contains_letters_repeated_separated_by_one_letter(string)
}

pub fn how_many_nice_strings_better_model(inputs: Vec<String>) -> u16 {
	let mut nice_string_counter: u16 = 0;

	for string in inputs {
		if is_nice_string_better_model(&string) {
			nice_string_counter += 1;
		}
	}

	nice_string_counter
}
