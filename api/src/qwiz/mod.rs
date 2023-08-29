pub mod routes;



use crate::POOL;
use crate::media::Media;
use serde::Deserialize;
use sqlx::types::Uuid;



#[derive(Deserialize)]
pub struct NewQwizData {
	pub name: String,
	pub creator_id: i32,
	pub thumbnail_uri: Option<String>
}

pub struct Qwiz {
	id: i32,
	name: String,
	pub creator_id: i32,
	thumbnail_uuid: Option<Uuid>,
}

impl Qwiz {

	pub async fn get_by_id(id: &i32) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Qwiz,
			"SELECT * FROM qwiz WHERE id=$1",
			id
		)
		.fetch_one(POOL.get().await)
		.await
	
	}

	pub async fn from_qwiz_data(data: &NewQwizData) -> Result<Self, sqlx::Error> {

		// check if creator uuid exists
		sqlx::query!(
			"SELECT id FROM account WHERE id=$1",
			&data.creator_id
		)
		.fetch_one(POOL.get().await)
		.await?;

		let thumbnail_uuid = match &data.thumbnail_uri {
			Some(uri) => Some(Media::new(uri).await?.uuid),
			None => None,
		};

		sqlx::query_as!(
			Qwiz,
			"INSERT INTO qwiz (name, creator_id, thumbnail_uuid) VALUES ($1, $2, $3) RETURNING *",
			&data.name,
			&data.creator_id,
			thumbnail_uuid,
		)
		.fetch_one(POOL.get().await)
		.await

	}

	pub async fn delete(self) -> Result<(), sqlx::Error> {

		sqlx::query!(
			"DELETE FROM qwiz WHERE id=$1",
			self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn update_name(&mut self, new_name: &String) -> Result<(), sqlx::Error> {

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
	pub async fn update_thumbnail_uri(&mut self, new_thumbnail_uri: &String) -> Result<(), sqlx::Error> {

		match self.thumbnail_uuid {
			Some(uuid) => Media::get_by_uuid(&uuid).await?.update(new_thumbnail_uri).await?,
			None => {
				
				let media = Media::new(new_thumbnail_uri).await?;
				
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

}
