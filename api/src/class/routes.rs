use crate::class::{Class, ClassError};

use rocket::{Route, serde::json::Json, http::Status, Either::{self, *}, response::status::BadRequest};
use serde::Serialize;



pub fn all() -> Vec<Route> {

	routes![
		class_info,
		get_class_by_id,
		get_classes_by_teacher_id,
	]

}



#[get("/class")]
fn class_info() -> &'static str {
r#"
GET /class/<id> - get class by id
GET /teacher/<id>/classes - get classes by teacher id
"#

}



#[derive(Serialize)]
struct GetClassData {
	id: i32,
	teacher_id: i32,
	student_ids: Vec<i32>,
}
impl GetClassData {

	async fn from_class(class: Class) -> sqlx::Result<Self> {
		Ok(
			Self {
				id: class.id,
				teacher_id: class.teacher_id,
				student_ids: class.get_all_students().await?,
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

#[get("/teacher/<teacher_id>/classes")]
async fn get_classes_by_teacher_id(teacher_id: i32) -> Result<Json<Vec<GetClassData>>, Either<Status, BadRequest<&'static str>>> {

	use ClassError::*;

	match Class::get_all_by_teacher_id(&teacher_id).await {
		Ok(classes) => {
			
			let mut class_datas: Vec<GetClassData> = Vec::new();

			for class in classes {
				match GetClassData::from_class(class).await {
					Ok(data) => class_datas.push(data),
					Err(e) => {

						eprintln!("{e}");
						return Err(Left(Status::InternalServerError))

					},
				}
			}

			Ok(Json(class_datas))

		},
		Err(Sqlx(e)) => {

			eprintln!("{e}");
			Err(Left(Status::InternalServerError))

		},
		Err(NotATeacher) => Err(Right(BadRequest(Some("Requested user is not a teacher")))),
		Err(TeacherNotFound) => Err(Left(Status::NotFound)),
	}

}