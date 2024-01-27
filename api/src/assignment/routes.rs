use crate::class::Class;
use crate::{internal_err, db_err_to_status};
use crate::assignment::Assignment;
use crate::account::{Account, AccountType};

use rocket::{Route, http::Status, serde::json::Json, Either::{*, self}, response::status::BadRequest};
use serde::{Serialize, Deserialize};



pub fn all() -> Vec<Route> {

	routes![
		get_account_assignments,
		create_assignment,
	]

}



#[derive(Serialize)]
struct GetAssignmentData {
	id: i32,
	qwiz_id: i32,
	qwiz_name: String,
	class_id: i32,
	open_time: Option<i64>,
	close_time: Option<i64>,
	completed: bool,
}
impl From<Assignment> for GetAssignmentData {
	fn from(value: Assignment) -> Self {
		Self {
			id: value.id,
			qwiz_id: value.qwiz_id,
			qwiz_name: value.qwiz_name.unwrap_or("None".to_owned()),
			class_id: value.class_id,
			open_time: value.open_time.map(|ts| ts.timestamp()),
			close_time: value.close_time.map(|ts| ts.timestamp()),
			completed: *value.completed,
		}
	}
}



#[derive(Deserialize)]
struct GetAssignmentsData {
	password: String,
}

#[post("/account/<id>/assignments", data = "<get_assignments_data>")]
async fn get_account_assignments(id: i32, get_assignments_data: Json<GetAssignmentsData>) -> Result<Json<Vec<GetAssignmentData>>, Either<Status, BadRequest<&'static str>>> {

	let mut account = match Account::get_by_id(&id).await {
		Ok(acc) => acc,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};

	match account.verify_password(&get_assignments_data.password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	}



	if !matches!(account.account_type, AccountType::Student | AccountType::Teacher) {
		return Err(Right(BadRequest(Some("not a teacher or student"))))
	}

	match Assignment::get_all_by_account_id(&account.id).await {
		Ok(assignments) => Ok(Json(assignments.into_iter().map(From::from).collect())),
		Err(e) => Err(Left(internal_err(&e))),
	}

}



#[derive(Deserialize)]
pub struct CreateAssignmentData {
	teacher_password: String,
	pub qwiz_id: i32,
	pub open_time: Option<i64>,
	pub close_time: Option<i64>,
}

#[post("/class/<id>/assignments", data = "<create_assignment_data>")]
async fn create_assignment(id: i32, create_assignment_data: Json<CreateAssignmentData>) -> Status {

	let class = match Class::get_by_id(&id).await {
		Ok(c) => c,
		Err(e) => return db_err_to_status(&e, Status::NotFound),
	};

	let mut account = match Account::get_by_id(&class.teacher_id).await {
		Ok(acc) => acc,
		Err(e) => return internal_err(&e),
	};

	match account.verify_password(&create_assignment_data.teacher_password).await {
		Ok(true) => (),
		Ok(false) => return Status::Unauthorized,
		Err(e) => return internal_err(&e),
	};

	match Assignment::create(id, &create_assignment_data).await {
		Ok(_) => Status::Created,
		Err(e) => internal_err(&e),
	}

}