pub mod routes;

use crate::POOL;
use crate::account::{Account, AccountType};

use serde::Deserialize;



#[derive(Deserialize)]
pub struct NewClassData {
	pub teacher_id: i32,
	pub student_ids: Option<Vec<i32>>,
}



pub enum ClassError {
	Sqlx(sqlx::Error),
	AccountNotFound(i32),
	NotATeacher(i32),
	NotAStudent(i32),
}
impl From<sqlx::Error> for ClassError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl ToString for ClassError {
	fn to_string(&self) -> String {
		
		use ClassError::*;

		match self {
			Sqlx(e) => e.to_string(),
			AccountNotFound(id) => format!("Account with id {id} not found"),
			NotAStudent(id) => format!("Account with id {id} is not a student"),
			NotATeacher(id) => format!("Account with id {id} is not a teacher"),
		}

	}
}



pub struct Class {
	pub id: i32,
	pub teacher_id: i32,
}

impl Class {

	pub async fn get_by_id(id: &i32) -> sqlx::Result<Self> {

		sqlx::query_as!(
			Class,
			"SELECT * FROM class WHERE id=$1",
			id,
		)
		.fetch_one(POOL.get().await)
		.await

	}
	pub async fn get_all_by_teacher_id(teacher_id: &i32) -> Result<Vec<Self>, ClassError> {

		if !Account::exists(teacher_id).await? {
			return Err(ClassError::AccountNotFound(*teacher_id))
		}

		if Account::get_by_id(teacher_id).await?.account_type != AccountType::Teacher {
			return Err(ClassError::NotATeacher(*teacher_id))
		}

		sqlx::query_as!(
			Class,
			"SELECT * FROM class WHERE teacher_id=$1",
			teacher_id,
		)
		.fetch_all(POOL.get().await)
		.await
		.map_err(From::from)
		

	}

	pub async fn from_class_data(data: &NewClassData) -> Result<Self, ClassError> {

		use ClassError::*;

		if !Account::exists(&data.teacher_id).await? {
			return Err(AccountNotFound(data.teacher_id))
		}

		if Account::get_by_id(&data.teacher_id).await?.account_type != AccountType::Teacher {
			return Err(NotATeacher(data.teacher_id))
		}

		let class = sqlx::query_as!(
			Class,
			"INSERT INTO class (teacher_id) VALUES ($1) RETURNING *",
			&data.teacher_id,
		)
		.fetch_one(POOL.get().await)
		.await?;

		if let Some(student_ids) = &data.student_ids {
			if let Err(e) = class.add_students(student_ids).await {

				class.delete().await?;
				return Err(e)
				
			}
		}

		Ok(class)

	}

	pub async fn delete(self) -> sqlx::Result<()> {

		sqlx::query!(
			"DELETE FROM class WHERE id=$1",
			self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}



	pub async fn get_all_students(&self) -> sqlx::Result<Vec<i32>> {

		Ok(
			sqlx::query!(
				"SELECT student_id FROM student WHERE class_id=$1",
				self.id,
			)
			.fetch_all(POOL.get().await)
			.await?
			.iter()
			.map(|r| r.student_id)
			.collect::<Vec<i32>>()
		)

	}
	
	pub async fn add_students(&self, new_student_ids: &Vec<i32>) -> Result<(), ClassError> {

		if new_student_ids.len() == 0 {
			return Ok(())
		}

		if let Some(id) = sqlx::query!(
			"SELECT id FROM account WHERE id=ANY($1) AND account_type!='student' ORDER BY id LIMIT 1",
			new_student_ids,
		)
		.fetch_optional(POOL.get().await)
		.await?
		.map(|r| r.id) {
			return Err(ClassError::NotAStudent(id))
		}

		sqlx::query!(
			"INSERT INTO student (student_id, class_id) VALUES (UNNEST($1::INTEGER[]), $2)",
			&new_student_ids,
			&self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

	pub async fn remove_students(&self, student_ids: &Vec<i32>) -> sqlx::Result<()> {

		sqlx::query!(
			"DELETE FROM student WHERE student_id=ANY($1) AND class_id=$2",
			&student_ids,
			&self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

}
