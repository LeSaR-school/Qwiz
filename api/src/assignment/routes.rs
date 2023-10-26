use crate::{internal_err, db_err_to_status};
use crate::assignment::Assignment;
use crate::account::{Account, AccountType};

use rocket::{Route, http::Status, serde::json::Json, Either::{*, self}, response::status::BadRequest};
use serde::{Serialize, Deserialize};



pub fn all() -> Vec<Route> {

	routes![
		get_account_assignments
	]

}



#[derive(Serialize)]
struct GetAssignmentData {
	qwiz_id: i32,
	class_id: i32,
	open_time: Option<i64>,
	close_time: Option<i64>,
	completed: bool,
}
impl From<Assignment> for GetAssignmentData {
	fn from(value: Assignment) -> Self {
		Self {
			qwiz_id: value.qwiz_id,
			class_id: value.class_id,
			open_time: value.open_time.map(|ts| ts.timestamp_millis()),
			close_time: value.close_time.map(|ts| ts.timestamp_millis()),
			completed: *value.completed,
		}
	}
}



#[derive(Deserialize)]
struct GetAssignmentsData {
	password: String,
}

#[get("/account/<id>/assignments", data = "<get_assignments_data>")]
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



	if account.account_type != AccountType::Student {
		return Err(Right(BadRequest(Some("not a student"))))
	}

	match Assignment::get_all_by_student_id(&account.id).await {
		Ok(assignments) => Ok(Json(assignments.into_iter().map(From::from).collect())),
		Err(e) => Err(Left(internal_err(&e))),
	}

}
