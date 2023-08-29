use sha2::{Sha512, Digest};
use rand::Rng;

static MAX_ITERATIONS: u32 = 127;

pub fn encode_password(password: String) -> String {

	let mut hasher = Sha512::new();
	hasher.update(password);
	let mut bytes: [u8; 64] = hasher.finalize().into();
	
	for _ in 0..rand::thread_rng().gen_range(0..MAX_ITERATIONS-1) {
		hasher = Sha512::new();
		hasher.update(bytes);
		bytes = hasher.finalize().into();
	}

	hex::encode(bytes)

}