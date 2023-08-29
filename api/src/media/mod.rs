use crate::POOL;

use uuid::Uuid;



pub async fn get_by_uuid(uuid: &Uuid) -> Result<String, sqlx::Error> {

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

pub async fn upload_multiple(urls: Vec<Option<String>>) -> Result<Vec<Option<Uuid>>, sqlx::Error> {

	let real_urls: Vec<String> = urls.iter().flat_map(|url| url.to_owned()).collect();

	let mut uuids = sqlx::query!(
		r#"INSERT INTO media (path)
		SELECT * FROM UNNEST($1::VARCHAR[])
		RETURNING uuid"#,
		&real_urls,
	).fetch_all(POOL.get().await)
	.await?
	.into_iter()
	.map(|r| r.uuid);

	let mut real_uuids: Vec<Option<Uuid>> = Vec::new();

	for url in urls {

		if url.is_some() {
			real_uuids.push(uuids.next());
		} else {
			real_uuids.push(None);
		}

	}

	Ok(real_uuids)

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
