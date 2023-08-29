use crate::POOL;
use crate::account::{Account, AccountType};

use serde::Deserialize;



#[derive(Deserialize)]
pub struct NewClassData {
	pub teacher_id: i32,
}



pub enum ClassError {
	Sqlx(sqlx::Error),
	TeacherNotFound,
	NotATeacher,
}
impl From<sqlx::Error> for ClassError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}

pub enum StudentError {
	Sqlx(sqlx::Error),
	StudentNotFound,
	NotAStudent,
}
impl From<sqlx::Error> for StudentError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
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
			return Err(ClassError::TeacherNotFound)
		}

		if Account::get_by_id(teacher_id).await?.account_type != AccountType::Teacher {
			return Err(ClassError::NotATeacher)
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

		if !Account::exists(&data.teacher_id).await? {
			return Err(ClassError::TeacherNotFound)
		}

		if Account::get_by_id(&data.teacher_id).await?.account_type != AccountType::Teacher {
			return Err(ClassError::NotATeacher)
		}

		sqlx::query_as!(
			Class,
			"INSERT INTO class (teacher_id) VALUES ($1) RETURNING *",
			&data.teacher_id,
		)
		.fetch_one(POOL.get().await)
		.await
		.map_err(From::from)

	}

	pub async fn update_teacher_id(&mut self, new_teacher_id: &i32) -> Result<(), ClassError> {

		if !Account::exists(new_teacher_id).await? {
			return Err(ClassError::TeacherNotFound)
		}

		if Account::get_by_id(new_teacher_id).await?.account_type != AccountType::Teacher {
			return Err(ClassError::NotATeacher)
		}

		self.teacher_id = sqlx::query!(
			"UPDATE class SET teacher_id=$1 WHERE id=$2 RETURNING teacher_id",
			new_teacher_id,
			&self.id,
		)
		.fetch_one(POOL.get().await)
		.await?
		.teacher_id;

		Ok(())

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
	
	pub async fn add_student(&self, new_student_id: &i32) -> Result<(), StudentError> {

		if !Account::exists(new_student_id).await? {
			return Err(StudentError::StudentNotFound);
		}

		if Account::get_by_id(new_student_id).await?.account_type != AccountType::Student {
			return Err(StudentError::NotAStudent);
		}

		sqlx::query!(
			"INSERT INTO student (student_id, class_id) VALUES ($1, $2) ON CONFLICT (student_id, class_id) DO NOTHING",
			new_student_id,
			&self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}
	
	pub async fn remove_student(&self, student_id: &i32) -> Result<(), StudentError> {

		if !Account::exists(student_id).await? {
			return Err(StudentError::StudentNotFound);
		}

		if Account::get_by_id(student_id).await?.account_type != AccountType::Student {
			return Err(StudentError::NotAStudent);
		}

		sqlx::query!(
			"DELETE FROM student WHERE student_id=$1 AND class_id=$2",
			student_id,
			&self.id,
		)
		.execute(POOL.get().await)
		.await?;

		Ok(())

	}

}