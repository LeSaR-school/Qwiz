pub mod routes;



use crate::POOL;
use crate::crypto;
use crate::media::Media;

use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;



#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
	Student,
	Parent,
	Teacher,
}

#[derive(Serialize)]
pub struct Account {
	pub id: i32,
	pub username: String,
	pub password_hash: String,
	pub profile_picture_uuid: Option<Uuid>,
	pub account_type: AccountType,
}

impl Account {

	pub async fn get_by_id(id: &i32) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Account,
			r#"SELECT id, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType" FROM account WHERE id=$1"#,
			id,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	pub async fn get_by_username(username: &String) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Account,
			r#"SELECT id, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType" FROM account WHERE username=$1"#,
			username,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	
	pub async fn new(username: &String, password: &String, account_type: &AccountType, profile_picture_uri: &Option<String>) -> Result<Self, sqlx::Error> {

		let password_hash = crypto::encode_password(password);
		let profile_picture_uuid = match profile_picture_uri {
			Some(uri) => Some(Media::new(uri).await?.uuid),
			None => None
		};

		sqlx::query_as!(
			Account,
			r#"INSERT INTO account (username, password_hash, account_type, profile_picture_uuid)
			VALUES ($1, $2, $3, $4) RETURNING id, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType""#,
			username,
			password_hash,
			account_type as &AccountType,
			profile_picture_uuid,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn delete(self) -> Result<(), sqlx::Error> {
		
		sqlx::query!(
			"DELETE FROM account WHERE id=$1",
			self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn update_password(&mut self, new_password: &String) -> Result<(), sqlx::Error> {

		let password_hash = crypto::encode_password(new_password);

		self.password_hash = sqlx::query!(
			"UPDATE account SET password_hash=$1 WHERE id=$2 RETURNING password_hash",
			password_hash,
			self.id
		)
		.fetch_one(POOL.get().await)
		.await?
		.password_hash;

		Ok(())

	}
	pub async fn update_account_type(&mut self, new_account_type: &AccountType) -> Result<(), sqlx::Error> {

		self.account_type = sqlx::query!(
			r#"UPDATE account SET account_type=$1 WHERE id=$2 RETURNING account_type AS "account_type!: AccountType""#,
			new_account_type as &AccountType,
			self.id
		)
		.fetch_one(POOL.get().await)
		.await?
		.account_type;

		Ok(())

	}
	pub async fn update_profile_picture_url(&mut self, new_profile_picture_uri: &String) -> Result<(), sqlx::Error> {

		match self.profile_picture_uuid {
			Some(uuid) => Media::get_by_uuid(&uuid).await?.update(new_profile_picture_uri).await?,
			None => {
				
				let media = Media::new(new_profile_picture_uri).await?;
				
				sqlx::query!(
					"UPDATE account SET profile_picture_uuid=$1 WHERE id=$2",
					media.uuid,
					self.id,
				)
				.execute(POOL.get().await)
				.await?;

				self.profile_picture_uuid = Some(media.uuid);

			},
		};

		Ok(())

	}

	pub async fn verify_password(&mut self, password: &String) -> Result<bool, sqlx::Error> {

		match crypto::verify_password(password, &self.password_hash).await {
			Ok(true) => self.update_password(password).await.map(|_| true),
			other => other,
		}

	}

}
