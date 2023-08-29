pub mod routes;

use crate::POOL;
use crate::qwiz::Qwiz;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct VoteData {
	pub voter_id: i32,
	pub qwiz_id: i32,
}

pub enum VoteError {
	QwizNotFound,
	SelfVote,
	Sqlx(sqlx::Error),
}
impl From<sqlx::Error> for VoteError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}

pub struct Vote {
	voter_id: i32,
	qwiz_id: i32,
}

impl Vote {

	pub async fn exists(voter_id: &i32, qwiz_id: &i32) -> Result<bool, VoteError> {

		sqlx::query!(
			"SELECT EXISTS(SELECT * FROM vote WHERE voter_id=$1 AND qwiz_id=$2)",
			voter_id,
			qwiz_id,
		)
		.fetch_one(POOL.get().await)
		.await
		.map(|r| r.exists.unwrap_or(false))
		.map_err(From::from)
		
	}

	pub async fn get_by_voter_id_qwiz_id(voter_id: &i32, qwiz_id: &i32) -> Result<Option<Self>, VoteError> {

		match Qwiz::exists(&qwiz_id).await {
			Ok(true) => (),
			Ok(false) => return Err(VoteError::QwizNotFound),
			Err(e) => return Err(VoteError::Sqlx(e)),
		};
	
		sqlx::query_as!(
			Vote,
			"SELECT * FROM vote WHERE voter_id=$1 AND qwiz_id=$2",
			voter_id,
			qwiz_id,
		)
		.fetch_optional(POOL.get().await)
		.await
		.map_err(From::from)
		
	}
	pub async fn get_all_by_qwiz_id(qwiz_id: &i32) -> Result<Vec<Self>, VoteError> {
		
		match Qwiz::exists(&qwiz_id).await {
			Ok(true) => (),
			Ok(false) => return Err(VoteError::QwizNotFound),
			Err(e) => return Err(VoteError::Sqlx(e)),
		};

		sqlx::query_as!(
			Vote,
			"SELECT * FROM vote WHERE qwiz_id=$1",
			qwiz_id,
		)
		.fetch_all(POOL.get().await)
		.await
		.map_err(From::from)
		
	}

	pub async fn from_vote_data(data: VoteData) -> Result<Self, VoteError> {

		match Qwiz::exists(&data.qwiz_id).await {
			Ok(true) => {
				if Qwiz::get_by_id(&data.qwiz_id).await?.creator_id == data.voter_id {
					return Err(VoteError::SelfVote);
				}
			},
			Ok(false) => return Err(VoteError::QwizNotFound),
			Err(e) => return Err(VoteError::Sqlx(e)),
		};

		sqlx::query_as!(
			Vote,
			r#"INSERT INTO vote (voter_id, qwiz_id) VALUES ($1, $2)
			ON CONFLICT (voter_id, qwiz_id) DO NOTHING
			RETURNING *"#,
			&data.voter_id,
			&data.qwiz_id,
		)
		.fetch_one(POOL.get().await)
		.await
		.map_err(From::from)

	}

	pub async fn delete(self) -> sqlx::Result<()> {

		sqlx::query!(
			"DELETE FROM vote WHERE voter_id=$1 AND qwiz_id=$2",
			&self.voter_id,
			&self.qwiz_id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

}