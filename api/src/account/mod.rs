pub mod routes;



use crate::POOL;
use crate::crypto;
use crate::media::{Media, NewMediaData, MediaError};
use std::fmt::Display;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;



#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
	Student,
	Parent,
	Teacher,
}



pub enum AccountError {
	Sqlx(sqlx::Error),
	Base64(base64::DecodeError),
	IO(std::io::Error),
}
impl Display for AccountError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		use AccountError::*;
		
		match self {
			Sqlx(e) => e.fmt(f),
			Base64(e) => e.fmt(f),
			IO(e) => e.fmt(f),
		}

	}
}
impl From<sqlx::Error> for AccountError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl From<MediaError> for AccountError {
	fn from(value: MediaError) -> Self {
		
		match value {
			MediaError::Sqlx(e) => AccountError::Sqlx(e),
			MediaError::Base64(e) => AccountError::Base64(e),
			MediaError::IO(e) => AccountError::IO(e),
		}

	}
}

pub enum NewAccountError {
	Base(AccountError),
	InvalidUsername,
	InvalidPassword,
	UsernameTaken,
}
impl From<AccountError> for NewAccountError {
	fn from(value: AccountError) -> Self {
		Self::Base(value)
	}
}
impl From<sqlx::Error> for NewAccountError {
	fn from(value: sqlx::Error) -> Self {
		Self::Base(AccountError::Sqlx(value))
	}
}
impl From<MediaError> for NewAccountError {
	fn from(value: MediaError) -> Self {
		Self::Base(AccountError::from(value))
	}
}
impl ToString for NewAccountError {
	fn to_string(&self) -> String {

		use NewAccountError::*;
		use AccountError::*;

		match self {
			Base(Sqlx(e)) => e.to_string(),
			Base(IO(e)) => e.to_string(),
			Base(Base64(_)) => "invalid base64".to_owned(),
			InvalidUsername => "invalid username".to_owned(),
			InvalidPassword => "invalid password".to_owned(),
			UsernameTaken => "username is taken".to_owned(),
		}

	}
}



lazy_static! {
	static ref IDS: Mutex<Vec<i32>> = Mutex::new(vec![]);
	static ref USERNAMES: Mutex<Vec<String>> = Mutex::new(vec![]);
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

	pub async fn load_cache() -> sqlx::Result<()> {

		let (db_ids, db_usernames): (Vec<i32>, Vec<String>) = sqlx::query!(
			"SELECT id, username FROM account"
		)
		.fetch_all(POOL.get().await)
		.await?
		.into_iter()
		.map(|r| (r.id, r.username))
		.unzip();

		println!("Loaded ids and usernames:\n{db_ids:?}\n{db_usernames:?}");
	
		*IDS.lock().await = db_ids;
		*USERNAMES.lock().await = db_usernames;

		Ok(())

	}

	async fn cache_id(id: &i32) {

		IDS.lock().await.push(*id)

	}
	async fn uncache_id(id: &i32) {

		IDS.lock().await.retain(|i| i != id)

	}

	async fn cache_username(username: &String) {

		USERNAMES.lock().await.push(username.to_owned())

	}
	async fn uncache_username(username: &String) {

		USERNAMES.lock().await.retain(|u| u != username)

	}



	pub async fn exists_id(id: &i32) -> bool {

		IDS.lock().await.contains(id)

	}
	pub async fn exists_username(username: &String) -> bool {

		USERNAMES.lock().await.contains(username)

	}
	
	pub async fn get_by_id(id: &i32) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Account,
			r#"SELECT id, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType" FROM account WHERE id=$1"#,
			id,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	pub async fn get_by_username(username: &String) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Account,
			r#"SELECT id, username, password_hash, profile_picture_uuid, account_type AS "account_type!: AccountType" FROM account WHERE username=$1"#,
			username,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	
	pub async fn new(username: &String, password: &String, account_type: &AccountType, profile_picture: &Option<NewMediaData>) -> Result<Self, NewAccountError> {

		// TODO: username and password filter
		// if () {
		// 	return Err(NewAccountError::InvalidUsername);
		// }
		// if () {
		// 	return Err(NewAccountError::InvalidPassword)
		// }

		if Self::exists_username(username).await {
			return Err(NewAccountError::UsernameTaken)
		}



		let password_hash = crypto::encode_password(password);
		let profile_picture_uuid = match profile_picture {
			Some(data) => Some(Media::from_media_data(data).await?.uuid),
			None => None
		};

		let account = sqlx::query_as!(
			Account,
			r#"INSERT INTO account (username, password_hash, account_type, profile_picture_uuid)
			VALUES ($1, $2, $3, $4) RETURNING id, username, password_hash, profile_picture_uuid, account_type AS "account_type!: _""#,
			username,
			password_hash,
			account_type as _,
			profile_picture_uuid,
		)
		.fetch_one(POOL.get().await)
		.await?;

		Self::cache_id(&account.id).await;
		Self::cache_username(&account.username).await;

		Ok(account)
	
	}

	pub async fn update_password(&mut self, new_password: &String) -> sqlx::Result<()> {

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
	pub async fn update_account_type(&mut self, new_account_type: &AccountType) -> sqlx::Result<()> {

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
	pub async fn update_profile_picture(&mut self, new_profile_picture: &NewMediaData) -> Result<(), AccountError> {

		match self.profile_picture_uuid {
			Some(uuid) => Media::get_by_uuid(&uuid).await?.update(new_profile_picture).await?,
			None => {
				
				let media = Media::from_media_data(new_profile_picture).await?;
				
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

	pub async fn delete(self) -> sqlx::Result<()> {
		
		sqlx::query!(
			"DELETE FROM account WHERE id=$1",
			&self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Self::uncache_id(&self.id).await;
		Self::uncache_username(&self.username).await;

		Ok(())

	}



	pub async fn verify_password(&mut self, password: &String) -> Result<bool, sqlx::Error> {

		match crypto::verify_password(password, &self.password_hash).await {
			Ok(true) => self.update_password(password).await.map(|_| true),
			other => other,
		}

	}

}
