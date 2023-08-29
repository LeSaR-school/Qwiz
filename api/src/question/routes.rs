use super::NewQuestionData;
use crate::{BASE_URL, media};
use crate::account::Account;
use crate::qwiz::Qwiz;
use crate::question::Question;
use crate::crypto::verify_password;
use rocket::{Route, Either};
use rocket::response::status::{Created, BadRequest};
use rocket::{http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		question_info,
		get_question_by_uuid_index,
		create_question,
		update_question,
		delete_question,
	]

}



#[get("/question")]
fn question_info() -> &'static str {
r#"
GET /question/<qwiz_uuid>/<index> - get question data by qwiz uuid and index

POST /question/<qwiz_uuid> - add a question to an existing qwiz
"creator_password": String - required
"question": {
	"body": String - required,
	"answer1": String - required,
	"answer2": String - required,
	"answer3": String - optional,
	"answer4": String - optional,
	"correct": 1 / 2 / 3 / 4 - required,
	"embed_url": String - optional,
} - required

PATCH /question/<qwiz_uuid>/<index> - update question data
"creator_password": String - required
"new_index": i32 - optional
"new_body": String - optional
"new answers": Vector of {
	"index"
} - optional
"new_embed_url": String - optional

DELETE /question/<qwiz_uuid> - delete question
"creator_password": String - required

"#
}



#[derive(Serialize)]
pub struct GetQuestionData {
	index: i32,
	body: String,
	answer1: String,
	asnwer2: String,
	answer3: Option<String>,
	answer4: Option<String>,
	correct: i32,
	embed_url: Option<String>,
}
impl GetQuestionData {
	
	pub async fn from_question(question: Question) -> Result<Self, sqlx::Error> {
		Ok(
			Self {
				index: question.index,
				body: question.body,
				answer1: question.answer1,
				asnwer2: question.answer2,
				answer3: question.answer3,
				answer4: question.answer4,
				correct: question.correct,
				embed_url: match question.embed_uuid {
					Some(uuid) => media::get_by_uuid(&uuid).await.ok(),
					None => None,
				},
			}
		)
	}

}

#[get("/question/<qwiz_uuid>/<index>")]
async fn get_question_by_uuid_index(qwiz_uuid: Uuid, index: i32) -> Result<Json<GetQuestionData>, Status> {

	match Question::get_by_qwiz_uuid_index(&qwiz_uuid, &index).await {
		Ok(question) => {
			match GetQuestionData::from_question(question).await {
				Ok(data) => Ok(Json(data)),
				Err(_) => Err(Status::InternalServerError),
			}
		},
		Err(_) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
pub struct PostQuestionData {
	pub creator_password: String,
	pub question: NewQuestionData,
}

#[post("/question/<qwiz_uuid>", data = "<question_data>")]
async fn create_question(qwiz_uuid: Uuid, question_data: Json<PostQuestionData>) -> Result<Created<String>, Status> {
	
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
				match Question::from_question_data(&qwiz_uuid, &question_data.question).await {
					Ok(question) => Ok(Created::new(format!("{}/question/{}/{}", BASE_URL, qwiz_uuid, question.index))),
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
	new_answers: Option<Vec<NewAnswer>>,
	new_correct: Option<i32>,
	new_embed_url: Option<String>,
}

#[patch("/question/<qwiz_uuid>/<index>", data = "<new_question_data>")]
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

						if let Some(new_answers) = &new_question_data.new_answers {
							for new_answer in new_answers {
								if question.update_answer(&new_answer.index, &new_answer.content).await.is_err() {
									return Err(Either::Right(BadRequest(Some("Bad answer"))));
								}
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

#[delete("/question/<qwiz_uuid>/<index>", data = "<delete_question_data>")]
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
