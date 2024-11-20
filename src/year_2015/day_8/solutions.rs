#![allow(dead_code)]

use regex::Regex;

trait Diff {
	fn get_string_size_diff(&self) -> usize;
}

#[derive(Debug)]
pub struct DecodeGift {
	memory_string: String,
	decode_string: String,
}

impl DecodeGift {
	fn parse(digital_gift_str: &str) -> DecodeGift {
		let memory_string: String = String::from(digital_gift_str);
		let mut decode_string: String = String::from(digital_gift_str);
		
		// Retire les guillemets au début et à la fin.
		decode_string = decode_string[1..decode_string.len()-1].to_string();
		
		decode_string = decode_string.replace("\\\\", "-");
		decode_string = decode_string.replace("\\\"", "@");
		let re = Regex::new(r"\\x[0-9a-fA-F][0-9a-fA-F]").unwrap();
		decode_string = re.replace_all(&decode_string, "_").to_string();
		
		DecodeGift {
			memory_string,
			decode_string,
		}
	}
}

impl Diff for DecodeGift {
	fn get_string_size_diff(&self) -> usize {
		self.memory_string.len() - self.decode_string.len()
	}
}


#[derive(Debug)]
struct EncodeGift {
	memory_string: String,
	encode_string: String,
}

impl EncodeGift {
	fn parse(digital_gift_str: &str) -> EncodeGift {
		let memory_string: String = String::from(digital_gift_str);
		let mut encode_string: String = String::from(digital_gift_str);
		
		encode_string = encode_string.replace("\\", "\\\\");
		encode_string = encode_string.replace("\"", "\\\"");
		encode_string = format!("\"{}\"", encode_string);
		
		EncodeGift {
			memory_string,
			encode_string,
		}
	}
}

impl Diff for EncodeGift {
	fn get_string_size_diff(&self) -> usize {
		self.encode_string.len() - self.memory_string.len()
	}
}

fn get_string_size_diff_sum<T: Diff>(gift_list: &Vec<T>) -> usize {
	let mut diffs: usize = 0;
	
	for i in 0..gift_list.len() {
		diffs += gift_list[i].get_string_size_diff();
	}
	
	diffs
}

pub fn get_decode_string_diff_sum(inputs: Vec<String>) -> usize {
	let mut decode_gift_list: Vec<DecodeGift> = Vec::new();
	
	for decode_gift_string in inputs {
		decode_gift_list.push(DecodeGift::parse(&decode_gift_string));
	}
	
	get_string_size_diff_sum(&decode_gift_list)
}

pub fn get_encode_string_diff_sum(inputs: Vec<String>) -> usize {
	let mut encode_gift_list: Vec<EncodeGift> = Vec::new();
	
	for encode_gift_string in inputs {
		encode_gift_list.push(EncodeGift::parse(&encode_gift_string));
	}
	
	get_string_size_diff_sum(&encode_gift_list)
}
