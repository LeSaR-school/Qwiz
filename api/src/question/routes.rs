use crate::{BASE_URL, db_err_to_status, internal_err};
use crate::account::Account;
use crate::qwiz::Qwiz;
use crate::question::{Question, NewQuestionData, QuestionError};
use crate::media::{Media, NewMediaData, routes::GetMediaData};
use rocket::{
	http::Status,
	serde::json::Json,
	response::status::{Created, BadRequest},
	Route,
	Either::{self, *}
};
use serde::{Serialize, Deserialize};



pub fn all() -> Vec<Route> {

	routes![
		question_info,
		get_question_by_qwiz_id_index,
		create_question,
		update_question,
		delete_question,
	]

}



#[get("/question")]
fn question_info() -> &'static str {
r#"
GET /question/<qwiz_id>/<index> - get question data by qwiz id and index

POST /question/<qwiz_id> - add a question to an existing qwiz
creator_password: String - required
question: {
	body: String - required,
	answer1: String - required,
	answer2: String - required,
	answer3: String - optional,
	answer4: String - optional,
	correct: 1/2/3/4 - required,
	embed: {
		data: String - required
		media_type: MediaType - required
	} - optional
} - required

PATCH /question/<qwiz_id>/<index> - update question data
creator_password: String - required
new_index: i32 - optional
new_body: String - optional
new_answers: Vector of {
	index: 1/2/3/4 - required,
	content: String - optional (null to delete)
} - optional
new_embed: {
	data: String - required
	media_type: MediaType - required
} - optional

DELETE /question/<qwiz_id> - delete question
creator_password: String - required
"#
}



#[derive(Serialize)]
pub struct GetQuestionData {
	index: i32,
	body: String,
	answer1: String,
	answer2: String,
	answer3: Option<String>,
	answer4: Option<String>,
	embed: Option<GetMediaData>,
	correct: u8,
}
impl GetQuestionData {
	
	pub async fn from_question(question: Question) -> Self {
		Self {
			index: question.index,
			body: question.body,
			answer1: question.answer1,
			answer2: question.answer2,
			answer3: question.answer3,
			answer4: question.answer4,
			embed: match question.embed_uuid {
				Some(uuid) => Media::get_by_uuid(&uuid).await.ok().map(Into::into),
				None => None,
			},
			correct: question.correct as u8,
		}
	}

}

#[get("/question/<qwiz_id>/<index>")]
async fn get_question_by_qwiz_id_index(qwiz_id: i32, index: i32) -> Result<Json<GetQuestionData>, Status> {

	match Question::get_by_qwiz_id_index(&qwiz_id, &index).await {
		Ok(question) => Ok(Json(GetQuestionData::from_question(question).await)),
		Err(e) => Err(db_err_to_status(&e, Status::NotFound)),
	}

}



#[derive(Deserialize)]
pub struct PostQuestionData {
	pub creator_password: String,
	pub question: NewQuestionData,
}

#[post("/question/<qwiz_id>", data = "<question_data>")]
async fn create_question(qwiz_id: i32, question_data: Json<PostQuestionData>) -> Result<Created<String>, Status> {
	
	let qwiz = match Qwiz::get_by_id(&qwiz_id).await {
		Ok(qwiz) => qwiz,
		Err(e) => return Err(db_err_to_status(&e, Status::NotFound)),
	};

	let mut account = match Account::get_by_id(&qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => return Err(internal_err(&e)),
	};

	match account.verify_password(&question_data.creator_password).await {
		Ok(true) => (),
		Ok(false) => return Err(Status::Unauthorized),
		Err(e) => return Err(internal_err(&e)),
	}



	match Question::from_question_data(&qwiz_id, &question_data.question).await {
		Ok(question) => Ok(Created::new(format!("{BASE_URL}/question/{qwiz_id}/{}", question.index))),
		Err(e) => Err(db_err_to_status(&e, Status::BadRequest)),
	}

}



#[derive(Deserialize)]
struct NewAnswer {
	index: u8,
	content: Option<String>,
}
#[derive(Deserialize)]
struct PatchQuestionData {
	creator_password: String,
	new_index: Option<i32>,
	new_body: Option<String>,
	new_answers: Option<Vec<NewAnswer>>,
	new_correct: Option<u8>,
	new_embed: Option<NewMediaData>,
}

#[patch("/question/<qwiz_id>/<index>", data = "<new_question_data>")]
async fn update_question(qwiz_id: i32, index: i32, mut new_question_data: Json<PatchQuestionData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	use QuestionError::*;

	let mut question = match Question::get_by_qwiz_id_index(&qwiz_id, &index).await {
		Ok(q) => q,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};

	let qwiz = match Qwiz::get_by_id(&qwiz_id).await {
		Ok(qwiz) => qwiz,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};

	let mut account = match Account::get_by_id(&qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => return Err(Left(internal_err(&e))),
	};

	match account.verify_password(&new_question_data.creator_password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	}
	


	if let Some(new_index) = &new_question_data.new_index {
		if question.update_index(new_index).await.is_err() {
			println!("a");
			return Err(Right(BadRequest(Some("bad index"))));
		}
	}

	if let Some(new_body) = &new_question_data.new_body {
		if question.update_body(new_body).await.is_err() {
			return Err(Right(BadRequest(Some("bad body"))));
		}
	}

	if let Some(new_correct) = &new_question_data.new_correct {
		if question.update_correct(new_correct).await.is_err() {
			return Err(Right(BadRequest(Some("bad correct"))));
		}
	}

	if let Some(new_answers) = &mut new_question_data.new_answers {
		new_answers.sort_by(|a, b| b.index.cmp(&a.index));
		for new_answer in new_answers {
			if question.update_answer(&new_answer.index, &new_answer.content).await.is_err() {
				return Err(Right(BadRequest(Some("bad answer"))));
			}
		}
	}

	if let Some(new_embed) = &new_question_data.new_embed {
		match question.update_embed(new_embed).await {
			Ok(_) => (),
			Err(Sqlx(e)) => return Err(Left(internal_err(&e))),
			Err(Base64(_)) => return Err(Right(BadRequest(Some("bad embed base64")))),
			Err(IO(e)) => return Err(Left(internal_err(&e))),
		}
	}

	Ok(Status::Ok)

}



#[derive(Deserialize)]
struct DeleteQuestionData {
	creator_password: String,
}

#[delete("/question/<qwiz_id>/<index>", data = "<delete_question_data>")]
async fn delete_question(qwiz_id: i32, index: i32, delete_question_data: Json<DeleteQuestionData>) -> Status {

	let question = match Question::get_by_qwiz_id_index(&qwiz_id, &index).await {
		Ok(q) => q,
		Err(e) => return db_err_to_status(&e, Status::NotFound),
	};

	let qwiz = match Qwiz::get_by_id(&qwiz_id).await {
		Ok(qwiz) => qwiz,
		Err(e) => return db_err_to_status(&e, Status::NotFound),
	};

	let mut account = match Account::get_by_id(&qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => return internal_err(&e),
	};

	match account.verify_password(&delete_question_data.creator_password).await {
		Ok(true) => (),
		Ok(false) => return Status::Unauthorized,
		Err(e) => return internal_err(&e),
	}



	match question.delete().await {
		Ok(_) => Status::Ok,
		Err(e) => internal_err(&e),
	}

}
