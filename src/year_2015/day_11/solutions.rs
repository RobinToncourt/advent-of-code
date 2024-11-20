#![allow(dead_code)]
pub struct Password {
	pass: String,
}

fn every_char_lowercase(input: &str) -> bool {
	input.chars().fold(true, |result, c| result && c.is_lowercase())
}

impl Password {
	pub fn parse(p: &str) -> Password {
		assert_eq!(p.len(), 8);
		assert!(every_char_lowercase(p));
		
		Password {
			pass: String::from(p),
		}
	}
	
	pub fn next_valid_password(&mut self) {
		let mut is_valid: bool = false;
		while !is_valid {
			is_valid = is_password_valid(&self.next().unwrap());
		}
	}
	
	pub fn get_password(&self) -> String {
		self.pass.clone()
	}
}

impl Iterator for Password {
	type Item = String;
	
	fn next(&mut self) -> Option<Self::Item> {
		let mut i: usize = self.pass.len()-1;
		let mut wraps_around: bool = true;
		
		let mut chars: Vec<char> = self.pass.chars().collect();
		while wraps_around {
			wraps_around = false;
			
			let mut c: char = chars[i];
			if c == 'z' {
				c = 'a';
				wraps_around = true;
			}
			else {
				c = char::from(c as u8 +1);
			}
			chars[i] = c;
			i -= 1;
		}
		
		self.pass = chars.iter().collect();
		Some(self.pass.clone())
	}
}

fn contains_three_increasing_consecutive_letters(input: &str) -> bool {
	let chars: Vec<char> = input.chars().collect();
	for i in 0..chars.len()-2 {
		let char_1: char = chars[i];
		let char_2: char = char::from((chars[i+1] as u8) -1);
		let char_3: char = char::from((chars[i+2] as u8) -2);
		
		if char_1 == char_2 && char_1 == char_3 {
			return true;
		}
	}
	
	false
}

fn contains_i_o_l_letter(input: &str) -> bool {
	input.contains('i') | input.contains('o') | input.contains('l')
}

fn contains_two_different_letters_pairs(input: &str) -> bool {
	let mut pairs: Vec<char> = Vec::new();
	
	let chars: Vec<char> = input.chars().collect();
	for i in 0..chars.len()-1 {
		let char_1: char = chars[i];
		let char_2: char = chars[i+1];
		
		if char_1 == char_2 {
			pairs.push(char_1);
		}
	}
	
	pairs.sort();
	pairs.dedup();
	pairs.len() >= 2
}

fn is_password_valid(p: &str) -> bool {
	contains_three_increasing_consecutive_letters(p) &&
	!contains_i_o_l_letter(p) &&
	contains_two_different_letters_pairs(p)
}
