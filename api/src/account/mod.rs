pub mod routes;



use crate::crypto;
use crate::media;
use crate::POOL;

use serde::{Serialize, Deserialize};
use sqlx::{types::Uuid, postgres::PgQueryResult};



#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
	Student,
	Parent,
	Teacher,
}

#[derive(Serialize)]
pub struct Account {
	pub uuid: Uuid,
	pub username: String,
	pub password_hash: String,
	pub profile_picture_uuid: Option<Uuid>,
	pub account_type: AccountType,
}

impl Account {

	pub async fn get_by_id(uuid: &Uuid) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Account,
			r#"SELECT uuid, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType" FROM account WHERE uuid=$1"#,
			uuid,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	pub async fn get_by_username(username: &String) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Account,
			r#"SELECT uuid, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType" FROM account WHERE username=$1"#,
			username,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	
	pub async fn new(username: &String, password: &String, account_type: &AccountType, profile_picture_url: &Option<String>) -> Result<Self, sqlx::Error> {

		let password_hash = crypto::encode_password(password);
		let profile_picture_uuid = match profile_picture_url {
			Some(url) => Some(media::upload(url).await?),
			None => None
		};

		sqlx::query_as!(
			Account,
			r#"INSERT INTO account (username, password_hash, account_type, profile_picture_uuid) VALUES ($1, $2, $3, $4) RETURNING uuid, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType""#,
			username,
			password_hash,
			account_type as &AccountType,
			profile_picture_uuid,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn delete(self) -> Result<PgQueryResult, sqlx::Error> {
		
		if let Some(uuid) = self.profile_picture_uuid {
			media::delete(uuid).await?;
		}

		sqlx::query!(
			"DELETE FROM account WHERE uuid=$1",
			self.uuid
		)
		.execute(POOL.get().await)
		.await

	}

	pub async fn update_password(&mut self, new_password: &String) -> Result<(), sqlx::Error> {

		let password_hash = crypto::encode_password(new_password);

		self.password_hash = sqlx::query!(
			"UPDATE account SET password_hash=$1 WHERE uuid=$2 RETURNING password_hash",
			password_hash,
			self.uuid
		)
		.fetch_one(POOL.get().await)
		.await?
		.password_hash;

		Ok(())

	}
	pub async fn update_account_type(&mut self, new_account_type: &AccountType) -> Result<(), sqlx::Error> {

		self.account_type = sqlx::query!(
			r#"UPDATE account SET account_type=$1 WHERE uuid=$2 RETURNING account_type AS "account_type!: AccountType""#,
			new_account_type as &AccountType,
			self.uuid
		)
		.fetch_one(POOL.get().await)
		.await?
		.account_type;

		Ok(())

	}
	pub async fn update_profile_picture_url(&mut self, new_profile_picture_url: &String) -> Result<PgQueryResult, sqlx::Error> {

		media::update(&mut self.profile_picture_uuid, new_profile_picture_url).await?;
		
		sqlx::query!(
			"UPDATE account SET profile_picture_uuid=$1 WHERE uuid=$2",
			self.profile_picture_uuid,
			self.uuid,
		)
		.execute(POOL.get().await)
		.await

	}

}
