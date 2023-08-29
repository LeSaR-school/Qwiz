use sqlx::{types::Uuid, Pool, Postgres, postgres::PgQueryResult};

struct Qwiz {
	uuid: Uuid,
	name: String,
	creator_uuid: Uuid,
	thumbnail_uuid: Option<Uuid>,
}

impl Qwiz {

	pub async fn get_by_id(uuid: Uuid, pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Qwiz,
			"SELECT * FROM qwiz WHERE uuid=$1",
			uuid
		).fetch_one(pool)
		.await
	
	}

	pub async fn create(name: String, creator_uuid: Uuid, thumbnail_url: Option<String>, pool: &Pool<Postgres>) -> Result<Self, sqlx::Error> {

		let thumbnail_uuid = match thumbnail_url {
			Some(url) => Some(
				sqlx::query!(
					"INSERT INTO media (path) VALUES ($1) RETURNING uuid",
					url
				).fetch_one(pool)
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
		).fetch_one(pool)
		.await

	}
	
	pub async fn delete(self, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {

		sqlx::query!(
			"DELETE FROM qwiz WHERE uuid=$1",
			self.uuid
		).execute(pool)
		.await

	}

	pub async fn update_name(&mut self, new_name: String, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

		self.name = sqlx::query!(
			"UPDATE qwiz SET name=$1 WHERE uuid=$2 RETURNING name",
			new_name,
			self.uuid,
		).fetch_one(pool)
		.await?
		.name;

		Ok(())

	}
	pub async fn update_thumbnail(&mut self, new_thumbnail_url: String, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {

		match self.thumbnail_uuid {
			Some(uuid) => {
				sqlx::query!(
					"UPDATE media SET path=$1 WHERE uuid=$2",
					new_thumbnail_url,
					uuid
				).execute(pool)
				.await?;
			},
			None => {
				self.thumbnail_uuid = Some(
					sqlx::query!(
						"INSERT INTO media (path) VALUES ($1) RETURNING uuid",
						new_thumbnail_url
					).fetch_one(pool)
					.await?
					.uuid
				);
			},
		};

		Ok(())

	}

}