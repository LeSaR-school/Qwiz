pub mod routes;

use crate::{POOL, media};

use sqlx::{types::Uuid, postgres::PgQueryResult};



pub struct Qwiz {
	uuid: Uuid,
	name: String,
	pub creator_uuid: Uuid,
	thumbnail_uuid: Option<Uuid>,
}

impl Qwiz {

	pub async fn get_by_uuid(uuid: &Uuid) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Qwiz,
			"SELECT * FROM qwiz WHERE uuid=$1",
			uuid
		)
		.fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn new(name: &String, creator_uuid: &Uuid, thumbnail_url: &Option<String>) -> Result<Self, sqlx::Error> {

		// check if creator uuid exists
		sqlx::query!(
			"SELECT uuid FROM account WHERE uuid=$1",
			creator_uuid
		)
		.fetch_one(POOL.get().await)
		.await?;

		let thumbnail_uuid = match thumbnail_url {
			Some(url) => Some(media::upload(url).await?),
			None => None,
		};

		sqlx::query_as!(
			Qwiz,
			"INSERT INTO qwiz (name, creator_uuid, thumbnail_uuid) VALUES ($1, $2, $3) RETURNING *",
			name,
			creator_uuid,
			thumbnail_uuid,
		)
		.fetch_one(POOL.get().await)
		.await

	}
	
	pub async fn delete(self) -> Result<(), sqlx::Error> {

		if let Some(uuid) = self.thumbnail_uuid {
			media::delete(uuid).await?;
		}

		sqlx::query!(
			"DELETE FROM qwiz WHERE uuid=$1",
			self.uuid,
		)
		.execute(POOL.get().await)
		.await?;

		sqlx::query!(
			r#"WITH deleted_uuids AS (
				DELETE FROM question WHERE qwiz_uuid=$1
				RETURNING embed_uuid
			)
			DELETE FROM media WHERE uuid IN (SELECT embed_uuid FROM deleted_uuids)"#,
			self.uuid,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn update_name(&mut self, new_name: &String) -> Result<(), sqlx::Error> {

		self.name = sqlx::query!(
			"UPDATE qwiz SET name=$1 WHERE uuid=$2 RETURNING name",
			new_name,
			self.uuid,
		)
		.fetch_one(POOL.get().await)
		.await?
		.name;

		Ok(())

	}
	pub async fn update_thumbnail_url(&mut self, new_thumbnail_url: &String) -> Result<PgQueryResult, sqlx::Error> {

		media::update(&mut self.thumbnail_uuid, new_thumbnail_url).await?;

		sqlx::query!(
			"UPDATE qwiz SET thumbnail_uuid=$1 WHERE uuid=$2",
			self.thumbnail_uuid,
			self.uuid,
		)
		.execute(POOL.get().await)
		.await

	}

}