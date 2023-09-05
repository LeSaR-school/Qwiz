pub mod routes;



use crate::POOL;
use crate::qwiz::routes::GetShortQwizData;
use crate::media::{Media, NewMediaData, MediaError};
use std::fmt::Display;
use serde::Deserialize;
use sqlx::types::Uuid;



#[derive(Deserialize)]
pub struct NewQwizData {
	pub name: String,
	pub creator_id: i32,
	pub thumbnail: Option<NewMediaData>,
	pub public: bool,
}



pub enum QwizError {
	Sqlx(sqlx::Error),
	Base64(base64::DecodeError),
	IO(std::io::Error),
}
impl Display for QwizError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		use QwizError::*;
		
		match self {
			Sqlx(e) => e.fmt(f),
			Base64(e) => e.fmt(f),
			IO(e) => e.fmt(f),
		}

	}
}
impl From<sqlx::Error> for QwizError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl From<base64::DecodeError> for QwizError {
	fn from(value: base64::DecodeError) -> Self {
		Self::Base64(value)
	}
}
impl From<std::io::Error> for QwizError {
	fn from(value: std::io::Error) -> Self {
		Self::IO(value)
	}
}
// impl From<MediaError> for QwizError {
// 	fn from(value: MediaError) -> Self {
		
// 		match value {
// 			MediaError::Sqlx(e) => QwizError::Sqlx(e),
// 			MediaError::Base64(e) => QwizError::Base64(e),
// 			MediaError::IO(e) => QwizError::IO(e),
// 		}

// 	}
// }



pub struct Qwiz {
	id: i32,
	name: String,
	pub creator_id: i32,
	thumbnail_uuid: Option<Uuid>,
	public: bool,
}

impl Qwiz {

	pub async fn exists(id: &i32) -> sqlx::Result<bool> {

		sqlx::query!(
			"SELECT EXISTS(SELECT * FROM qwiz WHERE id=$1)",
			id,
		)
		.fetch_one(POOL.get().await)
		.await
		.map(|r| r.exists.unwrap_or(false))

	}

	pub async fn get_by_id(id: &i32) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Qwiz,
			"SELECT * FROM qwiz WHERE id=$1",
			id
		)
		.fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn from_qwiz_data(data: &NewQwizData) -> Result<Self, MediaError> {

		// check if creator uuid exists
		sqlx::query!(
			"SELECT id FROM account WHERE id=$1",
			&data.creator_id
		)
		.fetch_one(POOL.get().await)
		.await?;

		let thumbnail_uuid = match &data.thumbnail {
			Some(data) => Some(Media::from_media_data(data).await?.uuid),
			None => None,
		};

		sqlx::query_as!(
			Qwiz,
			"INSERT INTO qwiz (name, creator_id, thumbnail_uuid, public) VALUES ($1, $2, $3, $4) RETURNING *",
			&data.name,
			&data.creator_id,
			thumbnail_uuid,
			&data.public,
		)
		.fetch_one(POOL.get().await)
		.await
		.map_err(From::from)

	}

	pub async fn delete(self) -> sqlx::Result<()> {

		sqlx::query!(
			"DELETE FROM qwiz WHERE id=$1",
			self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn update_name(&mut self, new_name: &String) -> sqlx::Result<()> {

		self.name = sqlx::query!(
			"UPDATE qwiz SET name=$1 WHERE id=$2 RETURNING name",
			new_name,
			self.id,
		)
		.fetch_one(POOL.get().await)
		.await?
		.name;

		Ok(())

	}
	pub async fn update_thumbnail(&mut self, new_thumbnail: &NewMediaData) -> Result<(), MediaError> {

		match self.thumbnail_uuid {
			Some(uuid) => Media::get_by_uuid(&uuid).await?.update(new_thumbnail).await?,
			None => {
				
				let media = Media::from_media_data(new_thumbnail).await?;
				
				sqlx::query!(
					"UPDATE qwiz SET thumbnail_uuid=$1 WHERE id=$2",
					media.uuid,
					self.id,
				)
				.execute(POOL.get().await)
				.await?;

				self.thumbnail_uuid = Some(media.uuid);

			},
		};

		Ok(())

	}



	pub async fn get_best(page: i64) -> sqlx::Result<Vec<GetShortQwizData>> {

		sqlx::query_as!(
			GetShortQwizData,
			r#"SELECT id, name,
			(SELECT uri FROM media WHERE uuid=thumbnail_uuid) AS thumbnail_uri,
			(SELECT COUNT(*) FROM vote WHERE qwiz_id=id) AS votes,
			(SELECT username FROM account WHERE id=creator_id) AS creator_name,
			(SELECT uri FROM media WHERE uuid=(SELECT profile_picture_uuid FROM account WHERE id=creator_id)) as creator_profile_picture_uri
			FROM qwiz
			ORDER BY votes LIMIT 50 OFFSET $1"#,
			page * 50,
		)
		.fetch_all(POOL.get().await)
		.await

	}

	pub async fn get_by_name(name: &String, page: i64) -> sqlx::Result<Vec<GetShortQwizData>> {
		sqlx::query_as!(
			GetShortQwizData,
			r#"SELECT id, name,
			(SELECT uri FROM media WHERE uuid=thumbnail_uuid) AS thumbnail_uri,
			(SELECT COUNT(*) FROM vote WHERE qwiz_id=id) AS votes,
			(SELECT username FROM account WHERE id=creator_id) AS creator_name,
			(SELECT uri FROM media WHERE uuid=(SELECT profile_picture_uuid FROM account WHERE id=creator_id)) as creator_profile_picture_uri
			FROM qwiz WHERE name LIKE $1
			ORDER BY votes LIMIT 50 OFFSET $2"#,
			format!("{name}%"),
			page * 50,
		)
		.fetch_all(POOL.get().await)
		.await
	}

}
