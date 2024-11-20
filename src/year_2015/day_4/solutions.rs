#![allow(dead_code)]

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InfiniteLoopFailSafe;

impl Error for InfiniteLoopFailSafe {}

impl fmt::Display for InfiniteLoopFailSafe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "To far...")
	}
}

fn hash_array_to_string(hash_array: [u8; 16]) -> String {
	let mut hash_string: String = String::new();

	hash_array.iter().for_each(
		|byte| hash_string.push_str(&format!("{:02x?}", byte))
	);

	hash_string
}

// Modifications dû au changement de crates.
fn get_md5_hash(text: &str) -> String {
	// let mut hasher = md5::new();
	// hasher.update(text.as_bytes());
	let digest = md5::compute(text.as_bytes());
	format!("{:x}", digest)
	// let result = hasher.finalize();
	// hash_array_to_string(result.into())
}

pub fn get_first_hash_start_five_zero
(secret_key: &str) -> Result<u32, InfiniteLoopFailSafe> {
	let mut number: u32 = 0;

	loop {
		let secret_key_number_hash: String
		= get_md5_hash(&[secret_key, &number.to_string()].join(""));
		
		if &secret_key_number_hash[0..5] == "00000" {
			return Ok(number);
		}

		if number >= 10_000_000 {
			return Err(InfiniteLoopFailSafe);
		}
		number += 1;
	}
}

pub fn get_first_hash_start_six_zero
(secret_key: &str) -> Result<u32, InfiniteLoopFailSafe> {
	let mut number: u32 = 0;

	loop {
		let secret_key_number_hash: String
		= get_md5_hash(&[secret_key, &number.to_string()].join(""));

		if &secret_key_number_hash[0..6] == "000000" {
			return Ok(number);
		}

		if number >= 10_000_000 {
			return Err(InfiniteLoopFailSafe);
		}

		number += 1;
	}
}
