pub mod routes;

use crate::POOL;

use sqlx::{types::Uuid, postgres::PgQueryResult};



pub struct Qwiz {
	uuid: Uuid,
	name: String,
	creator_uuid: Uuid,
	thumbnail_uuid: Option<Uuid>,
}

impl Qwiz {

	pub async fn get_by_id(uuid: &Uuid) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Qwiz,
			"SELECT * FROM qwiz WHERE uuid=$1",
			uuid
		).fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn new(name: &String, creator_uuid: &Uuid, thumbnail_url: &Option<String>) -> Result<Self, sqlx::Error> {

		// check if creator uuid exists
		sqlx::query!(
			"SELECT uuid FROM account WHERE uuid=$1",
			creator_uuid
		).fetch_one(POOL.get().await)
		.await?;

		let thumbnail_uuid = match thumbnail_url {
			Some(url) => Some(
				sqlx::query!(
					"INSERT INTO media (path) VALUES ($1) RETURNING uuid",
					url
				).fetch_one(POOL.get().await)
				.await?
				.uuid
			),
			None => None
		};

		sqlx::query_as!(
			Qwiz,
			"INSERT INTO qwiz (name, creator_uuid, thumbnail_uuid) VALUES ($1, $2, $3) RETURNING *",
			name,
			creator_uuid,
			thumbnail_uuid,
		).fetch_one(POOL.get().await)
		.await

	}
	
	pub async fn delete(self) -> Result<PgQueryResult, sqlx::Error> {

		sqlx::query!(
			"DELETE FROM qwiz WHERE uuid=$1",
			self.uuid
		).execute(POOL.get().await)
		.await

	}

	pub async fn update_name(&mut self, new_name: &String) -> Result<(), sqlx::Error> {

		self.name = sqlx::query!(
			"UPDATE qwiz SET name=$1 WHERE uuid=$2 RETURNING name",
			new_name,
			self.uuid,
		).fetch_one(POOL.get().await)
		.await?
		.name;

		Ok(())

	}
	pub async fn update_thumbnail(&mut self, new_thumbnail_url: &String) -> Result<(), sqlx::Error> {

		match self.thumbnail_uuid {
			Some(uuid) => {
				sqlx::query!(
					"UPDATE media SET path=$1 WHERE uuid=$2",
					new_thumbnail_url,
					uuid
				).execute(POOL.get().await)
				.await?;
			},
			None => {
				self.thumbnail_uuid = Some(
					sqlx::query!(
						"INSERT INTO media (path) VALUES ($1) RETURNING uuid",
						new_thumbnail_url
					).fetch_one(POOL.get().await)
					.await?
					.uuid
				);
			},
		};

		Ok(())

	}

}