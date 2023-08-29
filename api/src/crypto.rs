use sha2::{Sha512, Digest};
use rand::Rng;

use crate::account::Account;



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

pub async fn verify_password(password: &String, account: &mut Account) -> Result<bool, sqlx::Error> {

	let mut hasher = Sha512::new();
	hasher.update(password);
	let mut bytes: [u8; 64] = hasher.finalize().into();

	for _ in 0..MAX_ITERATIONS {

		hasher = Sha512::new();
		hasher.update(bytes);
		bytes = hasher.finalize().into();

		if hex::encode(bytes) == account.password_hash {

			// update account password hash after each verification
			account.update_password(password).await?;

			return Ok(true);

		};
		
	}

	Ok(false)
	
}