use sha2::{Sha512, Digest};
use rand::Rng;



static MAX_ITERATIONS: u32 = 127;

pub fn encode_password(password: &String) -> String {

	let mut hasher = Sha512::new();
	hasher.update(password);
	let mut bytes: [u8; 64] = hasher.finalize().into();

	let iterations = rand::thread_rng().gen_range(1..MAX_ITERATIONS);
	for _ in 0..iterations {

		hasher = Sha512::new();
		hasher.update(bytes);
		bytes = hasher.finalize().into();

	}

	hex::encode(bytes)

}

pub async fn verify_password(password: &String, password_hash: &String) -> sqlx::Result<bool> {

	let mut hasher = Sha512::new();
	hasher.update(password);
	let mut bytes: [u8; 64] = hasher.finalize().into();

	for _ in 0..MAX_ITERATIONS {

		hasher = Sha512::new();
		hasher.update(bytes);
		bytes = hasher.finalize().into();

		if hex::encode(bytes) == *password_hash {
			return Ok(true);
		}
		
	}

	Ok(false)
	
}

pub fn validate_password(password: &String) -> bool {

	if password.len() < 8 {
		return false
	}

	if !password.is_ascii() {
		return false
	}

	if password.as_bytes().iter().any(|c| *c < 33 || *c > 126) {
		return false
	}

	// no uppercase letters
	if &password.to_ascii_lowercase() == password {
		return false
	}

	// no lowercase letters
	if &password.to_ascii_uppercase() == password {
		return false
	}

	// no numbers
	if !password.as_bytes().iter().any(|c| *c >= 48 && *c < 58) {
		return false
	}

	// no special characters
	if !password.as_bytes().iter().any(|c| *c <= 47 || (*c >= 58 && *c <= 64) || (*c >= 91 && *c <= 96) || *c >= 123) {
		return false
	}

	true

}

pub fn validate_username(username: &String) -> bool {

	if username.len() < 3 {
		return false
	}

	if !username.is_ascii() {
		return false
	}

	if !username.as_bytes().iter().all(|c| (*c >= 48 && *c < 58) || (*c >= 65 && *c <= 90) || (*c >= 97 && *c <= 122) || *c == 95) {
		return false
	}

	true

}
