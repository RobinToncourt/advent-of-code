#[cfg(test)]
mod test {
	use crate::utils::utils::load_from_file;
	use super::super::solutions::{
		contains_3_vowels,
		contains_characters_twice_in_a_row,
		contains_forbidden_string,
		is_nice_string,
		how_many_nice_strings,
		contains_pair_without_overlapping_3,
		contains_letters_repeated_separated_by_one_letter,
		is_nice_string_better_model,
		how_many_nice_strings_better_model
	};

	// Part 1.

	#[test]
	fn test_contains_3_vowels_examples() {
		assert_eq!(contains_3_vowels("aei"), true);
		assert_eq!(contains_3_vowels("xazegov"), true);
		assert_eq!(contains_3_vowels("aeiouaeiouaeiou"), true);
	}

	#[test]
	fn test_contains_characters_twice_in_a_row_examples() {
		assert_eq!(contains_characters_twice_in_a_row("xx"), true);
		assert_eq!(contains_characters_twice_in_a_row("abcdde"), true);
		assert_eq!(contains_characters_twice_in_a_row("aabbccdd"), true);
	}

	#[test]
	fn test_contains_forbidden_string_examples() {
		assert_eq!(contains_forbidden_string("ab"), true);
		assert_eq!(contains_forbidden_string("cd"), true);
		assert_eq!(contains_forbidden_string("pq"), true);
		assert_eq!(contains_forbidden_string("xy"), true);
		assert_eq!(contains_forbidden_string("yxqpdcba"), false);
	}

	#[test]
	fn test_is_nice_string_examples() {
		assert_eq!(is_nice_string("ugknbfddgicrmopn"), true);
		assert_eq!(is_nice_string("aaa"), true);
		assert_eq!(is_nice_string("jchzalrnumimnmhp"), false);
		assert_eq!(is_nice_string("haegwjzuvuyypxyu"), false);
		assert_eq!(is_nice_string("dvszwmarrgswjxmb"), false);
	}

	#[test]
	fn test_is_nice_strings_puzzle_input() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_5/inputs.txt");
		assert_eq!(how_many_nice_strings(inputs), 238)
	}

	// Part 2.
	#[test]
	fn test_contains_pair_without_overlapping_examples() {
		assert_eq!(contains_pair_without_overlapping_3("xyxy"), true);
		assert_eq!(contains_pair_without_overlapping_3("aabcdefgaa"), true);
		assert_eq!(contains_pair_without_overlapping_3("aaa"), false);
	}

	#[test]
	fn test_contains_letters_repeated_separated_by_one_letter_examples() {
		assert_eq!(
			contains_letters_repeated_separated_by_one_letter("xyx"),
			true
		);
		assert_eq!(
			contains_letters_repeated_separated_by_one_letter("abcdefeghi"),
			true
		);
		assert_eq!(
			contains_letters_repeated_separated_by_one_letter("aaa"),
			true
		);
	}

	#[test]
	fn test_is_nice_string_better_model_examples() {
		assert_eq!(is_nice_string_better_model("qjhvhtzxzqqjkmpb"), true);
		assert_eq!(is_nice_string_better_model("xxyxx"), true);
		assert_eq!(is_nice_string_better_model("uurcxstgmygtbstg"), false);
		assert_eq!(is_nice_string_better_model("ieodomkazucvgmuy"), false);
	}

	#[test]
	fn test_is_nice_strings_better_model_puzzle_input() {
		let inputs: Vec<String> =
			load_from_file("./src/year_2015/day_5/inputs.txt");
		assert_eq!(how_many_nice_strings_better_model(inputs), 69);
	}

}
