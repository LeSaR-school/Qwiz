pub mod routes;



use crate::{POOL, BASE_URL};

use std::{path::Path, fs, io::Write, fmt::Display};
use base64::{engine::general_purpose, Engine};
use serde::{Serialize, Deserialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use uuid::Uuid;



#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, Copy)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
pub enum MediaType {
	Image,
	Video,
	Audio,
	Youtube,
}
impl MediaType {
	pub fn to_file_extension(&self) -> &'static str {

		use MediaType::*;

		match self {
			Image => "png",
			Video => "mp4",
			Audio => "mp3",
			Youtube => "",
		}

	}
}
impl PgHasArrayType for MediaType {
	fn array_type_info() -> PgTypeInfo {
		PgTypeInfo::with_name("_media_type")
	}
}



#[derive(Deserialize)]
pub struct NewMediaData {
	pub data: String,
	pub media_type: MediaType,
}
impl NewMediaData {

	async fn get_uri(&self) -> Result<String, MediaError> {

		use MediaType::*;
		use MediaError::*;

		if !matches!(self.media_type, Image | Video | Audio) {
			return Ok(self.data.to_owned())
		}

		let uuid = Uuid::new_v4();
		let path_str = format!("{}/media/{uuid}.{}", env!("CARGO_MANIFEST_DIR"), self.media_type.to_file_extension());
		let path = Path::new(&path_str);
		
		match general_purpose::STANDARD.decode(&self.data) {
			Ok(binary) => {

				match fs::OpenOptions::new()
					.create(true)
					.write(true)
					.open(path)
				{
					Ok(mut file) => {
						match file.write_all(&binary) {
							Ok(_) => Ok(format!("{BASE_URL}/media/upload/{uuid}.{}", self.media_type.to_file_extension())),
							Err(e) => Err(IO(e)),
						}
					},
					Err(e) => Err(IO(e)),
				}

			},
			Err(e) => Err(Base64(e)),
		}

	}

}



pub enum MediaError {
	Sqlx(sqlx::Error),
	Base64(base64::DecodeError),
	IO(std::io::Error),
}
impl Display for MediaError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		use MediaError::*;
		
		match self {
			Sqlx(e) => e.fmt(f),
			Base64(e) => e.fmt(f),
			IO(e) => e.fmt(f),
		}

	}
}
impl From<sqlx::Error> for MediaError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl From<base64::DecodeError> for MediaError {
	fn from(value: base64::DecodeError) -> Self {
		Self::Base64(value)
	}
}
impl From<std::io::Error> for MediaError {
	fn from(value: std::io::Error) -> Self {
		Self::IO(value)
	}
}



pub struct Media {
	pub uuid: Uuid,
	pub uri: String,
	pub media_type: MediaType,
}

impl Media {

	pub async fn get_by_uuid(uuid: &Uuid) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Media,
			r#"SELECT uuid, uri, media_type AS "media_type!: _" FROM media WHERE uuid=$1"#,
			uuid,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn from_media_data(data: &NewMediaData) -> Result<Self, MediaError> {

		let uri = data.get_uri().await?;

		sqlx::query_as!(
			Media,
			r#"INSERT INTO media (uri, media_type) VALUES ($1, $2)
			RETURNING uuid, uri, media_type AS "media_type!: _""#,
			&uri,
			data.media_type as _,
		)
		.fetch_one(POOL.get().await)
		.await
		.map_err(From::from)

	}
	
	pub async fn from_media_datas(media_datas: Vec<&Option<NewMediaData>>) -> Result<Vec<Option<Self>>, MediaError> {

		let uris = Self::upload_multiple(media_datas.iter().flat_map(|d| *d).collect()).await?;
		let media_types: Vec<MediaType> = media_datas.iter().flat_map(|d| *d).map(|d| d.media_type.clone()).collect();

		let mut medias = sqlx::query_as!(
			Media,
			r#"INSERT INTO media (uri, media_type)
			SELECT * FROM UNNEST($1::VARCHAR[], $2::media_type[])
			RETURNING uuid, uri, media_type AS "media_type!: _""#,
			&uris,
			media_types as _,
		)
		.fetch_all(POOL.get().await)
		.await?
		.into_iter();

		Ok(
			media_datas.into_iter().map(|data| {
				match data {
					Some(_) => medias.next(),
					None => None,
				}
			}).collect()
		)

	}
	async fn upload_multiple(datas: Vec<&NewMediaData>) -> Result<Vec<String>, MediaError> {

		let mut uris: Vec<String> = Vec::new();

		for data in datas {
			uris.push(data.get_uri().await?);
		}

		Ok(uris)

	}

	pub async fn update(&mut self, new_data: &NewMediaData) -> Result<(), MediaError> {

		let new_uri = new_data.get_uri().await?;

		sqlx::query!(
			"UPDATE media SET uri=$1, media_type=$2 WHERE uuid=$3",
			new_uri,
			new_data.media_type as _,
			self.uuid,
		)
		.execute(POOL.get().await)
		.await?;

		self.uri = new_uri;
		self.media_type = new_data.media_type;

		Ok(())
	
	}

}
