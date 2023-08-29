pub mod routes;



use crate::{POOL, BASE_URL};

use std::{path::Path, fs, io::Write};
use base64::{engine::general_purpose, Engine};
use rocket::Either;
use uuid::Uuid;



pub struct Media {
	pub uuid: Uuid,
	pub uri: String,
}

impl Media {

	pub async fn get_by_uuid(uuid: &Uuid) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Media,
			"SELECT * FROM media WHERE uuid=$1",
			uuid,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	
	pub async fn new(uri: &String) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Media,
			"INSERT INTO media (uri) VALUES ($1) RETURNING *",
			uri,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	pub async fn new_multiple(uris: Vec<&Option<String>>) -> Result<Vec<Option<Self>>, sqlx::Error> {

		let real_uris: Vec<String> = uris.clone().into_iter().flat_map(|uri| uri.to_owned()).collect();

		let mut medias = sqlx::query_as!(
			Media,
			r#"INSERT INTO media (uri)
			SELECT * FROM UNNEST($1::VARCHAR[])
			RETURNING *"#,
			&real_uris,
		)
		.fetch_all(POOL.get().await)
		.await?
		.into_iter();

		Ok(uris.into_iter().map(|uri| {
			match uri {
				Some(_) => medias.next(),
				None => None,
			}
		}).collect())

	}
	pub async fn upload(data: &String) -> Result<Self, Either<sqlx::Error, Either<base64::DecodeError, std::io::Error>>> {

		let uuid = Uuid::new_v4().to_string();
		let path_str = format!("{}/media/{uuid}", env!("CARGO_MANIFEST_DIR"));
		let path = Path::new(&path_str);
		
		match general_purpose::STANDARD.decode(data) {
			Ok(binary) => {

				match fs::OpenOptions::new()
					.create(true)
					.write(true)
					.open(&path)
				{
					Ok(mut file) => {
						
						file.write_all(&binary)
							.map_err(|e| Either::Right(Either::Right(e)))?;
						
						sqlx::query_as!(
							Media,
							"INSERT INTO media (uri) VALUES ($1) RETURNING *",
							format!("{BASE_URL}/media/{uuid}"),
						)
						.fetch_one(POOL.get().await)
						.await
						.map_err(|e| Either::Left(e))

					},
					Err(e) => return Err(Either::Right(Either::Right(e))),
				}

			},
			Err(e) => Err(Either::Right(Either::Left(e))),
		}

	}
	
	pub async fn update(&mut self, new_uri: &String) -> Result<(), sqlx::Error> {

		sqlx::query!(
			"UPDATE media SET uri=$1 WHERE uuid=$2",
			new_uri,
			self.uuid,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())
	
	}

}
