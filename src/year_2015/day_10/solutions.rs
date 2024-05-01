#![allow(dead_code)]
fn group_by_consecutive_duplicate(input: &str) -> Vec<Vec<char>> {
	let mut consecutive_duplicate_list: Vec<Vec<char>> = Vec::new();
	
	let mut consecutive_duplicate: Vec<char> = Vec::new();
	
	let char_vec: Vec<char> = input.chars().collect();
	let mut c: char = char_vec[0];
	
	consecutive_duplicate.push(c);
	
	for i in 1..char_vec.len() {
		if char_vec[i] != c {
			consecutive_duplicate_list.push(consecutive_duplicate);
			consecutive_duplicate = Vec::new();
		}
		
		consecutive_duplicate.push(char_vec[i]);
		c = char_vec[i];
	}
	consecutive_duplicate_list.push(consecutive_duplicate);
	
	consecutive_duplicate_list
}

fn create_next_sequence(consecutive_duplicate_list: Vec<Vec<char>>) -> String {
	let mut next_sequence: String =  String::new();
	
	for consecutive_duplicate in consecutive_duplicate_list {
		next_sequence.push_str(&consecutive_duplicate.len().to_string());
		next_sequence.push_str(&consecutive_duplicate[0].to_string());
	}
	
	next_sequence
}

pub fn look_and_say(input: &str) -> String {
	let consecutive_duplicate_list: Vec<Vec<char>> =
		group_by_consecutive_duplicate(input);
	create_next_sequence(consecutive_duplicate_list)
}

pub fn get_result_length(input: &str, nbr_of_process: usize) -> usize {
	
	let mut tmp: String = String::from(input);
	
	for _ in 0..nbr_of_process {
		tmp = look_and_say(&tmp);
	}
	
	tmp.len()
}