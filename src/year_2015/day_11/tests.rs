#[cfg(test)]
mod tests {
	use super::super::solutions::{
		Password,
	};
	
	const PREVIOUS_PASSWORD: &str = "hxbxwxba";
	
	#[test]
	fn test_get_next_valid_password_input() {
		let mut password: Password = Password::parse(PREVIOUS_PASSWORD);
		password.next_valid_password();
		
		assert_eq!(password.get_password(), "hxbxxyzz");
	}
	
	#[test]
	fn test_get_next_next_valid_password_input() {
		let mut password: Password = Password::parse(PREVIOUS_PASSWORD);
		password.next_valid_password();
		password.next_valid_password();
		
		assert_eq!(password.get_password(), "hxcaabcc");
	}
	
}