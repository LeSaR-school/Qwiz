pub mod routes;



use crate::POOL;
use crate::media::{Media, NewMediaData, MediaError};
use std::cmp::Ordering;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use uuid::Uuid;



#[derive(Deserialize)]
pub struct NewQuestionData {
	pub index: Option<i32>,
	pub body: String,
	pub answer1: String,
	pub answer2: String,
	pub answer3: Option<String>,
	pub answer4: Option<String>,
	pub correct: i16,
	pub embed_data: Option<NewMediaData>,
}



pub enum QuestionError {
	Sqlx(sqlx::Error),
	Base64(base64::DecodeError),
	IO(std::io::Error),
}
impl Display for QuestionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		use QuestionError::*;
		
		match self {
			Sqlx(e) => e.fmt(f),
			Base64(e) => e.fmt(f),
			IO(e) => e.fmt(f),
		}

	}
}
impl From<sqlx::Error> for QuestionError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl From<MediaError> for QuestionError {
	fn from(value: MediaError) -> Self {

		use MediaError::*;

		match value {
			Sqlx(e) => QuestionError::Sqlx(e),
			Base64(e) => QuestionError::Base64(e),
			IO(e) => QuestionError::IO(e),
		}

	}
}



#[derive(Serialize)]
pub struct Question {
	qwiz_id: i32,
	index: i32,
	body: String,
	answer1: String,
	answer2: String,
	answer3: Option<String>,
	answer4: Option<String>,
	correct: i16,
	embed_uuid: Option<Uuid>,
}

impl Question {

	pub async fn get_by_qwiz_id_index(qwiz_id: &i32, index: &i32) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Question,
			"SELECT * FROM question WHERE qwiz_id=$1 AND index=$2",
			qwiz_id,
			index,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	pub async fn get_all_by_qwiz_id(qwiz_id: &i32) -> sqlx::Result<Vec<Self>> {

		sqlx::query_as!(
			Question,
			"SELECT * FROM question WHERE qwiz_id=$1 ORDER BY index ASC",
			qwiz_id,
		)
		.fetch_all(POOL.get().await)
		.await
	
	}

	pub async fn from_question_data(qwiz_id: &i32, data: &NewQuestionData) -> sqlx::Result<Self> {

		let embed_uuid = match &data.embed_data {
			Some(d) => {
				match Media::from_media_data(d).await {
					Ok(m) => Some(m.uuid),
					Err(e) => {

						eprintln!("{e}");
						None

					},
				}
			},
			None => None
		};

		let real_index = match &data.index {
			Some(index) => {	
				// shift all existing questions after current index by 1
				sqlx::query!(
					"UPDATE question SET index=index+1 WHERE index>=$1 AND qwiz_id=$2 RETURNING index",
					&index,
					qwiz_id,
				)
				.fetch_one(POOL.get().await)
				.await?
				.index
			},
			None => {
				sqlx::query!(
					"SELECT MAX(index) + 1 AS max FROM question WHERE qwiz_id=$1",
					qwiz_id,
				)
				.fetch_one(POOL.get().await)
				.await?
				.max
				.unwrap_or(0)
			},
		};



		let question = sqlx::query_as!(
			Question,
			r#"INSERT INTO question (qwiz_id, index, body, answer1, answer2, answer3, answer4, correct, embed_uuid)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"#,
			qwiz_id,
			real_index,
			data.body,
			data.answer1,
			data.answer2,
			data.answer3,
			data.answer4,
			data.correct,
			embed_uuid,
		)
		.fetch_one(POOL.get().await)
		.await?;
	
		Ok(question)

	}
	pub async fn from_question_datas(qwiz_id: &i32, datas: &Vec<NewQuestionData>) -> Result<Vec<Self>, QuestionError> {

		let indexes: Vec<i32> = (0..datas.len()).map(|n| n as i32).collect();
		let bodies: Vec<String> = datas.iter().map(|d| d.body.to_owned()).collect();
		let answers1: Vec<String> = datas.iter().map(|d| d.answer1.to_owned()).collect();
		let answers2: Vec<String> = datas.iter().map(|d| d.answer2.to_owned()).collect();
		let answers3: Vec<String> = datas.iter().map(|d| d.answer3.to_owned().unwrap_or("".to_owned())).collect();
		let answers4: Vec<String> = datas.iter().map(|d| d.answer4.to_owned().unwrap_or("".to_owned())).collect();
		let corrects: Vec<i16> = datas.iter().map(|d| d.correct).collect();

		let medias: Vec<&Option<NewMediaData>> = datas.iter().map(|d| &d.embed_data).collect();
		let embed_uuids: Vec<Uuid> = Media::from_media_datas(medias)
			.await?
			.into_iter()
			.map(|m| m.map_or(Uuid::default(), |m| m.uuid))
			.collect();

		sqlx::query_as!(
			Question,
			r#"INSERT INTO question (qwiz_id, index, body, answer1, answer2, answer3, answer4, correct, embed_uuid)
			SELECT $1, index, body, answer1, answer2, NULLIF(answer3, ''), NULLIF(answer4, ''), correct, NULLIF(embed_uuid, uuid_nil())
			FROM UNNEST($2::INTEGER[], $3::VARCHAR[], $4::VARCHAR[], $5::VARCHAR[], $6::VARCHAR[], $7::VARCHAR[], $8::SMALLINT[], $9::UUID[])
			AS t(index, body, answer1, answer2, answer3, answer4, correct, embed_uuid)
			RETURNING *"#,
			qwiz_id,
			&indexes,
			&bodies,
			&answers1,
			&answers2,
			&answers3,
			&answers4,
			&corrects,
			&embed_uuids,
		)
		.fetch_all(POOL.get().await)
		.await
		.map_err(From::from)

	}

	pub async fn delete(self) -> sqlx::Result<()> {
		
		sqlx::query!(
			r#"WITH deleted AS (
				DELETE FROM question WHERE qwiz_id=$1 AND index=$2 RETURNING qwiz_id, index
			) UPDATE question SET index=index-1 WHERE index>(SELECT index FROM deleted) AND qwiz_id=(SELECT qwiz_id FROM deleted)"#,
			self.qwiz_id,
			self.index,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn update_index(&mut self, new_index: &i32) -> sqlx::Result<bool> {
		use Ordering::*;

		match new_index.cmp(&self.index) {
			Equal => Ok(false),
			Greater => {

				// temporarily delete the current question
				sqlx::query!(
					"DELETE FROM question WHERE qwiz_id=$1 AND index=$2",
					self.qwiz_id,
					self.index,
				)
				.execute(POOL.get().await)
				.await?;

				// shift all questions in (curr_idx; new_idx] down by 1
				sqlx::query!(
					"UPDATE question SET index=index-1 WHERE index>$1 AND index<=$2 AND qwiz_id=$3",
					self.index,
					new_index,
					self.qwiz_id,
				)
				.execute(POOL.get().await)
				.await?;

				// re-insert the current question at the new index
				self.index = sqlx::query!(
					r#"INSERT INTO question (qwiz_id, index, body, answer1, answer2, answer3, answer4, correct, embed_uuid)
					VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING index"#,
					self.qwiz_id,
					new_index,
					self.body,
					self.answer1,
					self.answer2,
					self.answer3,
					self.answer4,
					self.correct,
					self.embed_uuid,
				)
				.fetch_one(POOL.get().await)
				.await?
				.index;

				Ok(true)

			},
			Less => {

				// temporarily delete the current question
				sqlx::query!(
					r#"DELETE FROM question WHERE qwiz_id=$1 AND index=$2"#,
					self.qwiz_id,
					self.index,
				)
				.execute(POOL.get().await)
				.await?;

				// shift all questions in [new_idx; curr_idx) up by 1
				sqlx::query!(
					r#"UPDATE question SET index=index+1 WHERE index>=$1 AND index<$2 AND qwiz_id=$3"#,
					new_index,
					self.index,
					self.qwiz_id,
				)
				.execute(POOL.get().await)
				.await?;

				// re-insert the current question at the new index
				self.index = sqlx::query!(
					r#"INSERT INTO question (qwiz_id, index, body, answer1, answer2, answer3, answer4, embed_uuid)
					VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING index"#,
					self.qwiz_id,
					new_index,
					self.body,
					self.answer1,
					self.answer2,
					self.answer3,
					self.answer4,
					self.embed_uuid,
				)
				.fetch_one(POOL.get().await)
				.await?
				.index;

				Ok(true)

			},
		}

	}
	pub async fn update_body(&mut self, new_body: &String) -> sqlx::Result<()> {

		self.body = sqlx::query!(
			"UPDATE question SET body=$1 WHERE qwiz_id=$2 AND index=$3 RETURNING body",
			new_body,
			self.qwiz_id,
			self.index,
		)
		.fetch_one(POOL.get().await)
		.await?
		.body;

		Ok(())

	}
	pub async fn update_answer(&mut self, answer_number: &u8, new_answer: &Option<String>) -> sqlx::Result<bool> {

		match answer_number {
			1 => {
				if let Some(new_answer) = new_answer {
					self.answer1 = sqlx::query!(
						"UPDATE question SET answer1=$1 WHERE qwiz_id=$2 AND index=$3 RETURNING answer1",
						new_answer,
						self.qwiz_id,
						self.index,
					)
					.fetch_one(POOL.get().await)
					.await?
					.answer1;
				} else {
					return Ok(false);
				}
			},
			2 => {
				if let Some(new_answer) = new_answer {
					self.answer2 = sqlx::query!(
						"UPDATE question SET answer2=$1 WHERE qwiz_id=$2 AND index=$3 RETURNING answer2",
						new_answer,
						self.qwiz_id,
						self.index,
					)
					.fetch_one(POOL.get().await)
					.await?
					.answer2;
				} else {
					return Ok(false);
				}
			},
			3 => {
				self.answer3 = sqlx::query!(
					"UPDATE question SET answer3=$1 WHERE qwiz_id=$2 AND index=$3 RETURNING answer3",
					*new_answer,
					self.qwiz_id,
					self.index,
				)
				.fetch_one(POOL.get().await)
				.await?
				.answer3;
			},
			4 => {
				self.answer4 = sqlx::query!(
					r#"UPDATE question SET answer4=$1 WHERE qwiz_id=$2 AND index=$3 RETURNING answer4"#,
					*new_answer,
					self.qwiz_id,
					self.index,
				)
				.fetch_one(POOL.get().await)
				.await?
				.answer4;
			},
			_ => return Ok(false),
		};

		Ok(true)

	}
	pub async fn update_correct(&mut self, new_correct: &u8) -> sqlx::Result<()> {

		self.correct = sqlx::query!(
			"UPDATE question SET correct=$1 WHERE qwiz_id=$2 AND index=$3 RETURNING correct",
			*new_correct as i16,
			self.qwiz_id,
			self.index,
		)
		.fetch_one(POOL.get().await)
		.await?
		.correct;

		Ok(())

	}
	pub async fn update_embed(&mut self, new_data: &NewMediaData) -> Result<(), QuestionError> {

		match self.embed_uuid {
			Some(uuid) => Media::get_by_uuid(&uuid).await?.update(new_data).await?,
			None => {
				
				let media = Media::from_media_data(new_data).await?;
				
				sqlx::query!(
					"UPDATE question SET embed_uuid=$1 WHERE qwiz_id=$2 AND index=$3",
					media.uuid,
					self.qwiz_id,
					self.index,
				)
				.execute(POOL.get().await)
				.await?;

				self.embed_uuid = Some(media.uuid);

			},
		};

		Ok(())

	}

}
