use crate::{BASE_URL, internal_err, db_err_to_status};
use crate::account::Account;
use crate::class::{Class, NewClassData, ClassError};

use rocket::{Route, serde::json::Json, http::Status, Either::{self, *}, response::status::{BadRequest, Created}};
use serde::{Serialize, Deserialize};



pub fn all() -> Vec<Route> {

	routes![
		class_info,
		get_class_by_id,
		create_class,
		delete_class,
		add_students,
		get_account_classes,
	]

}



#[get("/class")]
fn class_info() -> &'static str {
r#"
GET /class/<id> - get class by id

POST /class - create a new class
teacher_password: String - required
class: {
	teacher_id: i32 - required
	name: String - required
	student_ids: Vec<i32> - optional
}

PUT /class/<id> - add students to a class
teacher_password: String - required
student_ids: Vec<i32> - required

DELETE /class/<id> - delete a class
teacher_password: String - required

DELETE /class/<id> - remove students from class
teacher_password: String - required
student_ids: Vec<i32> - required
"#

}



#[derive(Serialize)]
pub struct GetClassData {
	id: i32,
	name: String,
	teacher_id: i32,
	teacher_name: String,
	student_ids: Vec<i32>,
}
impl GetClassData {

	pub async fn from_class(class: Class) -> sqlx::Result<Self> {
		Ok(
			Self {
				student_ids: class.get_all_students().await?,
				id: class.id,
				name: class.name,
				teacher_id: class.teacher_id,
				teacher_name: class.teacher_name.unwrap_or("None".to_owned()),
			}
		)
	}

}

#[get("/class/<id>")]
async fn get_class_by_id(id: i32) -> Result<Json<GetClassData>, Status> {

	match Class::get_by_id(&id).await {
		Ok(class) => {
			match GetClassData::from_class(class).await {
				Ok(data) => Ok(Json(data)),
				Err(e) => Err(internal_err(&e)),
			}
		},
		Err(e) => Err(db_err_to_status(&e, Status::NotFound)),
	}

}



#[derive(Deserialize)]
struct PostClassData {
	teacher_password: String,
	class: NewClassData,
}

#[post("/class", data = "<class_data>")]
async fn create_class(class_data: Json<PostClassData>) -> Result<Created<String>, Either<Status, BadRequest<String>>> {

	use ClassError::*;

	let mut account = match Account::get_by_id(&class_data.class.teacher_id).await {
		Ok(acc) => acc,
		Err(e) => return Err(Left(internal_err(&e))),
	};

	match account.verify_password(&class_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	}



	match Class::from_class_data(&class_data.class).await {
		Ok(class) => Ok(Created::new(format!("{BASE_URL}/class/{}", class.id))),
		Err(Sqlx(e)) => Err(Left(internal_err(&e))),
		Err(e) => Err(Right(BadRequest(Some(e.to_string())))),
	}

}



#[derive(Deserialize)]
struct PutClassData {
	teacher_password: String,
	student_ids: Vec<i32>,
}

#[put("/class/<id>", data = "<put_class_data>")]
async fn add_students(id: i32, put_class_data: Json<PutClassData>) -> Either<Status, BadRequest<String>> {

	use ClassError::*;

	let class = match Class::get_by_id(&id).await {
		Ok(class) => class,
		Err(e) => return Left(db_err_to_status(&e, Status::NotFound)),
	};
	
	let mut account = match Account::get_by_id(&class.teacher_id).await {
		Ok(acc) => acc,
		Err(e) => return Left(internal_err(&e)),
	};

	match account.verify_password(&put_class_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Left(Status::Unauthorized),
		Err(e) => return Left(internal_err(&e)),
	}
	


	match class.add_students(&put_class_data.student_ids).await {
		Ok(_) => Left(Status::Ok),
		Err(Sqlx(e)) => Left(internal_err(&e)),
		Err(e) => Right(BadRequest(Some(e.to_string()))),
	}

}



#[derive(Deserialize)]
struct DeleteClassData {
	teacher_password: String,
	student_ids: Option<Vec<i32>>,
}

#[delete("/class/<id>", data = "<delete_class_data>")]
async fn delete_class(id: i32, delete_class_data: Json<DeleteClassData>) -> Either<Status, BadRequest<String>> {

	let class = match Class::get_by_id(&id).await {
		Ok(class) => class,
		Err(e) => return Left(db_err_to_status(&e, Status::NotFound)),
	};

	let mut account = match Account::get_by_id(&class.teacher_id).await {
		Ok(acc) => acc,
		Err(e) => return Left(internal_err(&e)),
	};

	match account.verify_password(&delete_class_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Left(Status::Unauthorized),
		Err(e) => return Left(internal_err(&e)),
	}



	if let Some(student_ids) = &delete_class_data.student_ids {

		match class.remove_students(student_ids).await {
			Ok(_) => Left(Status::Ok),
			Err(e) => Left(internal_err(&e)),
		}

	} else {

		match class.delete().await {
			Ok(_) => Left(Status::Ok),
			Err(e) => Left(internal_err(&e)),
		}

	}

}



#[derive(Deserialize)]
struct GetClassesData {
	password: String,
}

#[post("/account/<id>/classes", data = "<get_classes_data>")]
async fn get_account_classes(id: i32, get_classes_data: Json<GetClassesData>) -> Result<Json<Vec<GetClassData>>, Either<Status, BadRequest<String>>> {

	use ClassError::*;

	let mut account = match Account::get_by_id(&id).await {
		Ok(acc) => acc,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};

	match account.verify_password(&get_classes_data.password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	};



	let classes = match Class::get_all_by_student_id(&id).await {
		Ok(classes) => classes,
		Err(NotAStudent(_)) => {
					
			match Class::get_all_by_teacher_id(&id).await {
				Ok(classes) => classes,
				Err(Sqlx(e)) => return Err(Left(internal_err(&e))),
				Err(NotAStudent(id)) => return Err(Right(BadRequest(Some(format!("Account with id {id} is neither a teacher nor a student"))))),
				Err(e) => {
			
					eprintln!("unreachable, {}", e.to_string());
					return Err(Left(Status::InternalServerError))
		
				},
			}

		},
		Err(Sqlx(e)) => return Err(Left(internal_err(&e))),
		Err(AccountNotFound(_)) => return Err(Left(Status::NotFound)),
		Err(NotATeacher(id)) => {
			
			eprintln!("unreachable, {id}");
			return Err(Left(Status::InternalServerError))

		},
	};

	let mut class_datas: Vec<GetClassData> = Vec::new();

	for class in classes {
		match GetClassData::from_class(class).await {
			Ok(data) => class_datas.push(data),
			Err(e) => return Err(Left(internal_err(&e))),
		}
	}

	Ok(Json(class_datas))

}
