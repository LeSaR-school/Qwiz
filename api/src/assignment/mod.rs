pub mod routes;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{POOL, OptBool};
use sqlx::types::chrono::NaiveDateTime;



pub struct Assignment {
	id: i32,
	qwiz_id: i32,
	class_id: i32,
	open_time: Option<NaiveDateTime>,
	close_time: Option<NaiveDateTime>,
	completed: OptBool,
}

impl Assignment {

	pub async fn get_by_id(id: &i32) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Assignment,
			r#"SELECT *,
			EXISTS(SELECT * FROM completed_assignment WHERE assignment_id=id AND student_id=$1) AS completed
			FROM assignment WHERE id=$1"#,
			id,
		)
		.fetch_one(POOL.get().await)
		.await

	}

	pub async fn get_all_by_student_id(student_id: &i32) -> sqlx::Result<Vec<Self>> {

		sqlx::query_as!(
			Assignment,
			r#"SELECT *,
			EXISTS(SELECT * FROM completed_assignment WHERE assignment_id=id AND student_id=$1) AS completed
			FROM assignment
			WHERE class_id IN (SELECT class_id FROM student WHERE student_id=$1)"#,
			student_id,
		)
		.fetch_all(POOL.get().await)
		.await

	}



	pub async fn complete_by_student_id(&mut self, student_id: &i32) -> sqlx::Result<bool> {

		if *self.completed {
			return Ok(true);
		}

		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time went backwards")
			.as_millis();

		if let Some(close_time) = self.close_time {
			if (close_time.timestamp_millis() as u128) < now {
				return Ok(false)
			}
		}
		if let Some(open_time) = self.open_time {
			if (open_time.timestamp_millis() as u128) > now {
				return Ok(false)
			}
		}

		sqlx::query!(
			r#"INSERT INTO completed_assignment (assignment_id, student_id) VALUES ($1, $2)
			ON CONFLICT (assignment_id, student_id) DO NOTHING"#,
			self.id,
			student_id,
		)
		.execute(POOL.get().await)
		.await?;

		self.completed = true.into();

		Ok(true)

	}

}