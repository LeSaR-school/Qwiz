use crate::{BASE_URL, internal_err, db_err_to_status};
use crate::assignment::Assignment;
use crate::account::Account;
use crate::qwiz::{Qwiz, QwizSolveError};
use crate::question::{Question, NewQuestionData, routes::GetQuestionData, QuestionError};
use crate::media::{Media, NewMediaData, routes::GetMediaData, MediaError};
use crate::qwiz::NewQwizData;

use rocket::{
	http::Status,
	serde::json::Json,
	response::status::{BadRequest, Created},
	Route,
	Either::{self, *},
};
use serde::{Deserialize, Serialize};



pub fn all() -> Vec<Route> {

	routes![
		qwiz_info,
		get_qwiz_by_id,
		get_best,
		get_recent,
		create_qwiz,
		update_qwiz,
		delete_qwiz,
		solve_qwiz,
	]

}



#[get("/qwiz")]
fn qwiz_info() -> &'static str {
r#"
GET /qwiz/<id> - get qwiz data by id

GET /qwiz/best?<page>&<search> - get 50 best qwizes by name, rated by votes

GET /qwiz/recent?<page> - get 50 best qwizes created in the last 2 weeks, rated by votes

POST /qwiz - create a qwiz
creator_password: String - required
qwiz: {
	name: String - required
	creator_id: i32 - required
	thumbnail_uri: String - optional
	public: bool - optional
} - required
questions: Vector of {
	body: String - required,
	answer1: String - required,
	answer2: String - required,
	answer3: String - optional,
	answer4: String - optional,
	correct: 1/2/3/4 - required,
	embed: {
		data: String - required
		media_type: MediaType - required
	} - optional,
} - required

PATCH /qwiz/<id> - update qwiz data
creator_password: String - required
new_name: String - optional
new_thumbnail: String - optional

DELETE /qwiz/<id> - delete qwiz
creator_password: String - required

POST /qwiz/<id>/solve - solve qwiz

answers: Vec<1/2/3/4> - required
"#
}



#[derive(Serialize)]
struct GetFullQwizData {
	id: i32,
	name: String,
	creator_id: i32,
	thumbnail: Option<GetMediaData>,
	questions: Vec<GetQuestionData>,
	public: bool,
	create_time: i64,
}
impl GetFullQwizData {

	async fn from_qwiz(qwiz: Qwiz) -> sqlx::Result<Self> {

		let mut questions: Vec<GetQuestionData> = Vec::new();
		for question in Question::get_all_by_qwiz_id(&qwiz.id).await? {
			questions.push(GetQuestionData::from_question(question).await);
		}

		Ok(
			Self {
				id: qwiz.id,
				name: qwiz.name,
				creator_id: qwiz.creator_id,
				thumbnail: match qwiz.thumbnail_uuid {
					Some(uuid) => Media::get_by_uuid(&uuid).await.ok().map(Into::into),
					None => None,
				},
				questions,
				public: qwiz.public,
				create_time: qwiz.create_time.timestamp_millis(),
			}
		)

	}

}

#[get("/qwiz/<id>")]
async fn get_qwiz_by_id(id: i32) -> Result<Json<GetFullQwizData>, Status> {

	match Qwiz::get_by_id(&id).await {
		Ok(qwiz) => {
			match GetFullQwizData::from_qwiz(qwiz).await {
				Ok(data) => Ok(Json(data)),
				Err(e) => Err(internal_err(&e)),
			}
		},
		Err(e) => Err(db_err_to_status(&e, Status::NotFound)),
	}

}



#[derive(Serialize)]
pub struct GetShortQwizData {
	pub id: i32,
	pub name: String,
	pub thumbnail_uri: Option<String>,
	pub votes: Option<i64>,
	pub creator_name: Option<String>,
	pub creator_profile_picture_uri: Option<String>,
	pub create_time: Option<i64>,
}

#[get("/qwiz/best?<page>&<search>")]
async fn get_best(page: Option<u32>, search: Option<String>) -> Result<Json<Vec<GetShortQwizData>>, Status> {

	match search {
		Some(s) => {
			match Qwiz::get_by_name(&s, page.unwrap_or(0) as i64).await {
				Ok(datas) => Ok(Json(datas)),
				Err(e) => Err(db_err_to_status(&e, Status::BadRequest)),
			}
		},
		None => {
			match Qwiz::get_best(page.unwrap_or(0) as i64).await {
				Ok(datas) => Ok(Json(datas)),
				Err(e) => Err(db_err_to_status(&e, Status::BadRequest)),
			}
		},
	}

}

#[get("/qwiz/recent?<page>")]
async fn get_recent(page: Option<u32>) -> Result<Json<Vec<GetShortQwizData>>, Status> {
	
	match Qwiz::get_recent(14, page.unwrap_or(0) as i64).await {
		Ok(datas) => Ok(Json(datas)),
		Err(e) => Err(db_err_to_status(&e, Status::BadRequest)),
	}

}



#[derive(Deserialize)]
struct PostQwizData {
	creator_password: String,
	qwiz: NewQwizData,
	questions: Vec<NewQuestionData>,
}

#[post("/qwiz", data = "<qwiz_data>")]
async fn create_qwiz(qwiz_data: Json<PostQwizData>) -> Result<Created<String>, Either<Status, BadRequest<&'static str>>> {

	use MediaError::*;

	let mut account = match Account::get_by_id(&qwiz_data.qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(_) => return Err(Left(Status::Unauthorized)),
	};

	match account.verify_password(&qwiz_data.creator_password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	}



	let qwiz = match Qwiz::from_qwiz_data(&qwiz_data.qwiz).await {

		Ok(qwiz) => qwiz,
		Err(Sqlx(e)) => return Err(Left(db_err_to_status(&e, Status::BadRequest))),
		Err(Base64(_)) => return Err(Right(BadRequest(Some("bad thumbnail base64")))),
		Err(IO(e)) => return Err(Left(internal_err(&e))),

	};

	if let Err(e) = Question::from_question_datas(&qwiz.id, &qwiz_data.questions).await {

		use QuestionError::*;

		if let Err(e) = qwiz.delete().await {
			return Err(Left(internal_err(&e)));
		}
		match e {
			Sqlx(e) => return Err(Left(db_err_to_status(&e, Status::BadRequest))),
			Base64(_) => return Err(Right(BadRequest(Some("bad media base64")))),
			IO(e) => return Err(Left(internal_err(&e))),
		}

	}

	Ok(Created::new(format!("{BASE_URL}/qwiz/{}", qwiz.id)))

}



#[derive(Deserialize)]
struct PatchQwizData {
	creator_password: String,
	new_name: Option<String>,
	new_thumbnail: Option<NewMediaData>,
}

#[patch("/qwiz/<id>", data = "<new_qwiz_data>")]
async fn update_qwiz(id: i32, new_qwiz_data: Json<PatchQwizData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	use MediaError::*;

	let mut qwiz = match Qwiz::get_by_id(&id).await {
		Ok(q) => q,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};

	let mut account = match Account::get_by_id(&qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => return Err(Left(internal_err(&e))),
	};

	match account.verify_password(&new_qwiz_data.creator_password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	}



	if let Some(new_name) = &new_qwiz_data.new_name {
		if qwiz.update_name(new_name).await.is_err() {
			return Err(Right(BadRequest(Some("bad name"))));
		}
	}

	if let Some(new_thumbnail) = &new_qwiz_data.new_thumbnail {
		match qwiz.update_thumbnail(new_thumbnail).await {
			Ok(_) => (),
			Err(Sqlx(e)) => return Err(Left(internal_err(&e))),
			Err(Base64(_)) => return Err(Right(BadRequest(Some("bad thumbnail base64")))),
			Err(IO(e)) => return Err(Left(internal_err(&e))),
		}
	}

	Ok(Status::Ok)

}



#[derive(Deserialize)]
struct DeleteQwizData {
	creator_password: String,
}

#[delete("/qwiz/<id>", data = "<delete_qwiz_data>")]
async fn delete_qwiz(id: i32, delete_qwiz_data: Json<DeleteQwizData>) -> Status {

	let qwiz = match Qwiz::get_by_id(&id).await {
		Ok(qwiz) => qwiz,
		Err(e) => return db_err_to_status(&e, Status::NotFound),
	};

	let mut account = match Account::get_by_id(&qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => return internal_err(&e),
	};

	match account.verify_password(&delete_qwiz_data.creator_password).await {
		Ok(true) => (),
		Ok(false) => return Status::Unauthorized,
		Err(e) => return internal_err(&e),
	}



	match qwiz.delete().await {
		Ok(_) => Status::Ok,
		Err(e) => internal_err(&e),
	}

}



#[derive(Deserialize)]
pub struct PostSolveQwizData {
	pub answers: Vec<u8>,
	username: Option<String>,
}

#[derive(Serialize)]
struct SolveQwizData {
	correct: u32,
	total: u32,
	results: Vec<bool>,
	assignment_complete: Option<bool>,
}

#[post("/qwiz/<qwiz_id>/solve?<assignment_id>", data = "<solve_qwiz_data>")]
async fn solve_qwiz(qwiz_id: i32, solve_qwiz_data: Json<PostSolveQwizData>, assignment_id: Option<i32>) -> Result<Json<SolveQwizData>, Either<Status, BadRequest<&'static str>>> {

	use QwizSolveError::*;

	let qwiz = match Qwiz::get_by_id(&qwiz_id).await {
		Ok(qwiz) => qwiz,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};

	let results = match qwiz.solve(&solve_qwiz_data.answers).await {
		Ok(results) => results,
		Err(Sqlx(e)) => return Err(Left(internal_err(&e))),
		Err(e) => return Err(Right(BadRequest(Some(e.as_str())))),
	};



	match (assignment_id, &solve_qwiz_data.username) {
		(Some(id), Some(username)) => {
			
			let student = match Account::get_by_username(username).await {
				Ok(acc) => acc,
				Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
			};
			
			let mut assignment = match Assignment::get_by_id(&id).await {
				Ok(assignment) => assignment,
				Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
			};

			let solved = results.iter().all(|r| *r);

			if solved {
				if let Err(e) = assignment.complete_by_student_id(&student.id).await {
					return Err(Left(internal_err(&e)))
				}
			}

			Ok(Json(SolveQwizData { correct: results.iter().filter(|&r| *r).count() as u32, total: results.len() as u32, results, assignment_complete: Some(solved) }))

		},
		(Some(_), None) => Err(Right(BadRequest(Some("no username")))),
		(_, _) => Ok(Json(SolveQwizData { correct: results.iter().filter(|&r| *r).count() as u32, total: results.len() as u32, results, assignment_complete: None })),
 	}

}
