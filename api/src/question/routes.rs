use crate::account::Account;
use crate::qwiz::Qwiz;
use crate::question::Question;
use crate::crypto::verify_password;
use rocket::{Route, Either};
use rocket::response::status::{Created, BadRequest};
use rocket::{http::Status, serde::json::Json};
use serde::Deserialize;
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		new_question,
		update_question,
		delete_question,
	]

}



#[derive(Deserialize)]
pub struct PostQuestionData {
	pub creator_password: String,
	pub body: String,
	pub index: Option<i32>,
	pub answer1: String,
	pub answer2: String,
	pub answer3: Option<String>,
	pub answer4: Option<String>,
	pub correct: i32,
	pub embed_url: Option<String>,
}

#[post("/qwiz/<qwiz_uuid>", data = "<question_data>")]
async fn new_question(qwiz_uuid: Uuid, question_data: Json<PostQuestionData>) -> Result<Created<String>, Status> {
	
	let qwiz = match Qwiz::get_by_uuid(&qwiz_uuid).await {
		Ok(qwiz) => qwiz,
		Err(_) => return Err(Status::NotFound),
	};

	let mut account = match Account::get_by_id(&qwiz.creator_uuid).await {
		Ok(acc) => acc,
		Err(_) => return Err(Status::InternalServerError),
	};

	match verify_password(&question_data.creator_password, &mut account).await {
		Ok(verified) => {
			if verified {
				match Question::from_question_data(&qwiz_uuid, &question_data).await {
					Ok(question) => Ok(Created::new(format!("{}/{}", qwiz_uuid, question.index))),
					Err(_) => Err(Status::BadRequest),
				}
			} else {
				Err(Status::Unauthorized)
			}
		},
		Err(_) => Err(Status::InternalServerError),
	}

}



#[derive(Deserialize)]
struct NewAnswer {
	index: u8,
	content: String,
}
#[derive(Deserialize)]
struct PatchQuestionData {
	creator_password: String,
	new_index: Option<i32>,
	new_body: Option<String>,
	new_answer: Option<NewAnswer>,
	new_correct: Option<i32>,
	new_embed_url: Option<String>,
}

#[patch("/qwiz/<qwiz_uuid>/<index>", data = "<new_question_data>")]
async fn update_question(qwiz_uuid: Uuid, index: i32, new_question_data: Json<PatchQuestionData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	match Question::get_by_qwiz_uuid_index(&qwiz_uuid, &index).await {
		Ok(mut question) => {

			let qwiz = match Qwiz::get_by_uuid(&qwiz_uuid).await {
				Ok(qwiz) => qwiz,
				Err(_) => return Err(Either::Left(Status::NotFound)),
			};
		
			let mut account = match Account::get_by_id(&qwiz.creator_uuid).await {
				Ok(acc) => acc,
				Err(_) => return Err(Either::Left(Status::InternalServerError)),
			};

			match verify_password(&new_question_data.creator_password, &mut account).await {
				Ok(verified) => {
					if verified {

						if let Some(new_index) = &new_question_data.new_index {
							if question.update_index(new_index).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad index"))));
							}
						}

						if let Some(new_body) = &new_question_data.new_body {
							if question.update_body(new_body).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad body"))));
							}
						}

						if let Some(new_answer) = &new_question_data.new_answer {
							if question.update_answer(new_answer.index, &new_answer.content).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad answer"))));
							}
						}

						if let Some(new_correct) = &new_question_data.new_correct {
							if question.update_correct(new_correct).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad correct"))));
							}
						}

						if let Some(new_embed_url) = &new_question_data.new_embed_url {
							if question.update_embed_url(new_embed_url).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad embed url"))));
							}
						}

						Ok(Status::Ok)

					} else {
						Err(Either::Left(Status::Unauthorized))
					}
				},
				Err(_) => Err(Either::Left(Status::InternalServerError)),
			}

		},
		Err(_) => Err(Either::Left(Status::NotFound)),
	}

}



#[derive(Deserialize)]
struct DeleteQuestionData {
	creator_password: String,
}

#[delete("/qwiz/<qwiz_uuid>/<index>", data = "<delete_question_data>")]
async fn delete_question(qwiz_uuid: Uuid, index: i32, delete_question_data: Json<DeleteQuestionData>) -> Status {

	match Question::get_by_qwiz_uuid_index(&qwiz_uuid, &index).await {
		Ok(question) => {
			
			let qwiz = match Qwiz::get_by_uuid(&qwiz_uuid).await {
				Ok(qwiz) => qwiz,
				Err(_) => return Status::NotFound,
			};
		
			let mut account = match Account::get_by_id(&qwiz.creator_uuid).await {
				Ok(acc) => acc,
				Err(_) => return Status::InternalServerError,
			};

			match verify_password(&delete_question_data.creator_password, &mut account).await {
				Ok(verified) => {
					if verified {
						match question.delete().await {
							Ok(_) => Status::Ok,
							Err(_) => Status::InternalServerError,
						}
					} else {
						Status::Unauthorized
					}
				},
				Err(_) => Status::InternalServerError,
			}

		},
		Err(_) => Status::NotFound,
	}

}
