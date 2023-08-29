use serde::Serialize;
use sqlx::{types::Uuid, Pool, Postgres, postgres::PgQueryResult};

use crate::crypto;



#[derive(Serialize)]
pub struct Account {
	pub uuid: Uuid,
	pub username: String,
	pub password_hash: String,
	pub profile_picture_uuid: Option<Uuid>,
}

impl Account {

	pub async fn get_by_id(uuid: Uuid, pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Account,
			"SELECT * FROM account WHERE uuid=$1",
			uuid
		).fetch_one(pool)
		.await
	
	}

	pub async fn new(username: &String, password: &String, pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {

		let password_hash = crypto::encode_password(password);
		
		sqlx::query_as!(
			Account,
			"INSERT INTO account (username, password_hash) VALUES ($1, $2) RETURNING *",
			username,
			password_hash,
		).fetch_one(pool)
		.await
	
	}

	pub async fn delete(self, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
		
		sqlx::query!(
			"DELETE FROM account WHERE uuid=$1",
			self.uuid
		).execute(pool)
		.await

	}

	pub async fn update_password(&mut self, new_password: &String, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

		let password_hash = crypto::encode_password(new_password);

		self.password_hash = sqlx::query!(
			"UPDATE account SET password_hash=$1 WHERE uuid=$2 RETURNING password_hash",
			password_hash,
			self.uuid
		).fetch_one(pool)
		.await?
		.password_hash;

		Ok(())

	}

}
