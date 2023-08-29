pub mod routes;

use crate::POOL;
use crate::media;

use serde::Serialize;
use uuid::Uuid;

use self::routes::PostQuestionData;

#[derive(Serialize)]
pub struct Question {
	qwiz_uuid: Uuid,
	index: i32,
	body: String,
	answer1: String,
	answer2: String,
	answer3: Option<String>,
	answer4: Option<String>,
	correct: i32,
	embed_uuid: Option<Uuid>,
}

impl Question {

	pub async fn get_by_qwiz_uuid_index(qwiz_uuid: &Uuid, index: &i32) -> Result<Self, sqlx::Error> {

		sqlx::query_as!(
			Question,
			r#"SELECT * FROM question WHERE qwiz_uuid=$1 AND index=$2"#,
			qwiz_uuid,
			index,
		)
		.fetch_one(POOL.get().await)
		.await
	
	}
	pub async fn get_all_by_qwiz_uuid(qwiz_uuid: &Uuid) -> Result<Vec<Self>, sqlx::Error> {

		sqlx::query_as!(
			Question,
			r#"SELECT * FROM question WHERE qwiz_uuid=$1 ORDER BY index ASC"#,
			qwiz_uuid,
		)
		.fetch_all(POOL.get().await)
		.await
	
	}
	
	pub async fn new(qwiz_uuid: &Uuid, body: &String, answer1: &String, answer2: &String, answer3: &Option<String>, answer4: &Option<String>, correct: &i32, embed_url: &Option<String>, index: &Option<i32>) -> Result<Self, sqlx::Error> {

		let embed_uuid = match embed_url {
			Some(url) => Some(media::upload(url).await?),
			None => None
		};

		// shift all existing questions after current index by 1
		sqlx::query!(
			r#"UPDATE question SET index=index+1 WHERE index>=$1 AND qwiz_uuid=$2"#,
			*index,
			qwiz_uuid,
		)
		.execute(POOL.get().await)
		.await?;

		let mut question = sqlx::query_as!(
			Question,
			r#"INSERT INTO question (qwiz_uuid, index, body, answer1, answer2, answer3, answer4, correct, embed_uuid)
			VALUES ($1, (SELECT MAX(index) FROM question WHERE qwiz_uuid=$1) + 1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#,
			qwiz_uuid,
			body,
			answer1,
			answer2,
			*answer3,
			*answer4,
			correct,
			embed_uuid,
		)
		.fetch_one(POOL.get().await)
		.await?;
	
		if let Some(i) = &index {
			question.update_index(i).await?;
		};

		Ok(question)

	}
	pub async fn from_question_data(qwiz_uuid: &Uuid, question_data: &PostQuestionData) -> Result<Self, sqlx::Error> {

		Self::new(&qwiz_uuid, &question_data.body, &question_data.answer1, &question_data.answer2, &question_data.answer3, &question_data.answer4, &question_data.correct, &question_data.embed_url, &question_data.index).await

	}

	pub async fn delete(self) -> Result<(), sqlx::Error> {
		
		if let Some(uuid) = self.embed_uuid {
			media::delete(uuid).await?;
		}

		sqlx::query!(
			"DELETE FROM question WHERE qwiz_uuid=$1 AND index=$2",
			self.qwiz_uuid,
			self.index,
		)
		.execute(POOL.get().await)
		.await?;

		sqlx::query!(
			"UPDATE question SET index=index-1 WHERE index>$1 AND qwiz_uuid=$2",
			self.index,
			self.qwiz_uuid,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn update_index(&mut self, new_index: &i32) -> Result<bool, sqlx::Error> {

		if *new_index == self.index {
			Ok(false)
		} else if *new_index > self.index {

			// temporarily delete the current question
			sqlx::query!(
				"DELETE FROM question WHERE qwiz_uuid=$1 AND index=$2",
				self.qwiz_uuid,
				self.index,
			)
			.execute(POOL.get().await)
			.await?;

			// shift all questions in (curr_idx; new_idx] down by 1
			sqlx::query!(
				r#"UPDATE question SET index=index-1 WHERE index>$1 AND index<=$2 AND qwiz_uuid=$3"#,
				self.index,
				new_index,
				self.qwiz_uuid,
			)
			.execute(POOL.get().await)
			.await?;

			// re-insert the current question at the new index
			self.index = sqlx::query!(
				r#"INSERT INTO question (qwiz_uuid, index, body, answer1, answer2, answer3, answer4, correct, embed_uuid)
				VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING index"#,
				self.qwiz_uuid,
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

		} else {

			// temporarily delete the current question
			sqlx::query!(
				r#"DELETE FROM question WHERE qwiz_uuid=$1 AND index=$2"#,
				self.qwiz_uuid,
				self.index,
			)
			.execute(POOL.get().await)
			.await?;

			// shift all questions in [new_idx; curr_idx) up by 1
			sqlx::query!(
				r#"UPDATE question SET index=index+1 WHERE index>=$1 AND index<$2 AND qwiz_uuid=$3"#,
				new_index,
				self.index,
				self.qwiz_uuid,
			)
			.execute(POOL.get().await)
			.await?;

			// re-insert the current question at the new index
			self.index = sqlx::query!(
				r#"INSERT INTO question (qwiz_uuid, index, body, answer1, answer2, answer3, answer4, embed_uuid)
				VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING index"#,
				self.qwiz_uuid,
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

		}

	}
	pub async fn update_body(&mut self, new_body: &String) -> Result<(), sqlx::Error> {

		self.body = sqlx::query!(
			"UPDATE question SET body=$1 WHERE qwiz_uuid=$2 AND index=$3 RETURNING body",
			new_body,
			self.qwiz_uuid,
			self.index,
		)
		.fetch_one(POOL.get().await)
		.await?
		.body;

		Ok(())

	}
	pub async fn update_answer(&mut self, answer_number: u8, new_answer: &String) -> Result<bool, sqlx::Error> {

		match answer_number {
			1 => {
				self.answer1 =  sqlx::query!(
					"UPDATE question SET answer1=$1 WHERE qwiz_uuid=$2 AND index=$3 RETURNING answer1",
					new_answer,
					self.qwiz_uuid,
					self.index,
				)
				.fetch_one(POOL.get().await)
				.await?
				.answer1;
			},
			2 => {
				self.answer2 =  sqlx::query!(
					"UPDATE question SET answer2=$1 WHERE qwiz_uuid=$2 AND index=$3 RETURNING answer2",
					new_answer,
					self.qwiz_uuid,
					self.index,
				)
				.fetch_one(POOL.get().await)
				.await?
				.answer2;
			},
			3 => {
				self.answer3 =  sqlx::query!(
					"UPDATE question SET answer3=$1 WHERE qwiz_uuid=$2 AND index=$3 RETURNING answer3",
					new_answer,
					self.qwiz_uuid,
					self.index,
				)
				.fetch_one(POOL.get().await)
				.await?
				.answer3;
			},
			4 => {
				self.answer4 =  sqlx::query!(
					r#"UPDATE question SET answer4=$1 WHERE qwiz_uuid=$2 AND index=$3 RETURNING answer4"#,
					new_answer,
					self.qwiz_uuid,
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
	pub async fn update_correct(&mut self, new_correct: &i32) -> Result<(), sqlx::Error> {

		self.correct = sqlx::query!(
			"UPDATE question SET correct=$1 WHERE qwiz_uuid=$2 AND index=$3 RETURNING correct",
			new_correct,
			self.qwiz_uuid,
			self.index,
		)
		.fetch_one(POOL.get().await)
		.await?
		.correct;

		Ok(())

	}
	pub async fn update_embed_url(&mut self, new_embed_url: &String) -> Result<(), sqlx::Error> {

		media::update(&mut self.embed_uuid, new_embed_url).await?;

		sqlx::query!(
			"UPDATE question SET embed_uuid=$1 WHERE qwiz_uuid=$2 AND index=$3",
			self.embed_uuid,
			self.qwiz_uuid,
			self.index,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

}