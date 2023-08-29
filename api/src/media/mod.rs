pub mod routes;

use crate::POOL;

use uuid::Uuid;



pub async fn get(uuid: &Uuid) -> Result<String, sqlx::Error> {

	sqlx::query!(
		"SELECT path FROM media WHERE uuid=$1",
		uuid,
	)
	.fetch_one(POOL.get().await)
	.await
	.map(|r| r.path)

}

pub async fn upload(url: &String) -> Result<Uuid, sqlx::Error> {

	sqlx::query!(
		"INSERT INTO media (path) VALUES ($1) RETURNING uuid",
		url
	).fetch_one(POOL.get().await)
	.await
	.map(|r| r.uuid)

}

pub async fn update(uuid: &mut Option<Uuid>, new_url: &String) -> Result<(), sqlx::Error> {

	*uuid = Some(
		match uuid {
			Some(id) => sqlx::query!(
				"UPDATE media SET path=$1 WHERE uuid=$2 RETURNING uuid",
				new_url,
				*id
			)
			.fetch_one(POOL.get().await)
			.await?
			.uuid,
			None => sqlx::query!(
				"INSERT INTO media (path) VALUES ($1) RETURNING uuid",
				new_url
			)
			.fetch_one(POOL.get().await)
			.await?
			.uuid,
		}
	);

	Ok(())

}

pub async fn delete(uuid: Uuid) -> Result<String, sqlx::Error> {

	sqlx::query!(
		"DELETE FROM media WHERE uuid=$1 RETURNING path",
		uuid
	)
	.fetch_one(POOL.get().await)
	.await
	.map(|r| r.path)

}
