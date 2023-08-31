use crate::BASE_URL;
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
	teacher_id: i32,
	name: String,
	student_ids: Vec<i32>,
}
impl GetClassData {

	pub async fn from_class(class: Class) -> sqlx::Result<Self> {
		Ok(
			Self {
				id: class.id,
				teacher_id: class.teacher_id,
				student_ids: class.get_all_students().await?,
				name: class.name,
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
				Err(e) => {

					eprintln!("{e}");
					Err(Status::InternalServerError)

				},
			}
		},
		Err(_) => Err(Status::NotFound),
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
		Err(e) => {

			eprintln!("{e}");
			return Err(Left(Status::InternalServerError))
			
		},
	};

	match account.verify_password(&class_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => {

			eprintln!("{e}");
			return Err(Left(Status::InternalServerError))
			
		},
	}



	match Class::from_class_data(&class_data.class).await {
		Ok(class) => {

			Ok(Created::new(format!("{BASE_URL}/class/{}", class.id)))

		},
		Err(Sqlx(e)) => {

			eprintln!("{e}");
			Err(Left(Status::InternalServerError))

		},
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
		Err(e) => {

			eprintln!("{e}");
			return Left(Status::InternalServerError)

		},
	};
	
	let mut account = match Account::get_by_id(&class.teacher_id).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Left(Status::InternalServerError)
			
		},
	};

	match account.verify_password(&put_class_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Left(Status::Unauthorized),
		Err(e) => {

			eprintln!("{e}");
			return Left(Status::InternalServerError)
			
		},
	}
	


	match class.add_students(&put_class_data.student_ids).await {
		Ok(_) => Left(Status::Ok),
		Err(Sqlx(e)) => {

			eprintln!("{e}");
			Left(Status::InternalServerError)

		},
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
		Err(e) => {

			eprintln!("{e}");
			return Left(Status::InternalServerError)

		},
	};

	let mut account = match Account::get_by_id(&class.teacher_id).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Left(Status::InternalServerError)
			
		},
	};

	match account.verify_password(&delete_class_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Left(Status::Unauthorized),
		Err(e) => {

			eprintln!("{e}");
			return Left(Status::InternalServerError)
			
		},
	}



	if let Some(student_ids) = &delete_class_data.student_ids {

		match class.remove_students(student_ids).await {
			Ok(_) => Left(Status::Ok),
			Err(e) => {

				eprintln!("{e}");
				Left(Status::InternalServerError)

			},
		}

	} else {

		match class.delete().await {
			Ok(_) => Left(Status::Ok),
			Err(e) => {

				eprintln!("{e}");
				Left(Status::InternalServerError)

			},
		}

	}

}
