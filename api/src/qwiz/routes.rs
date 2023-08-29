use crate::BASE_URL;
use crate::account::Account;
use crate::qwiz::{Qwiz, QwizError};
use crate::question::{Question, NewQuestionData, routes::GetQuestionData};
use crate::media::{Media, NewMediaData, routes::GetMediaData};

use rocket::{
	http::Status,
	serde::json::Json,
	response::status::{BadRequest, Created},
	Route,
	Either::{self, *},
};
use serde::{Deserialize, Serialize};

use super::NewQwizData;



pub fn all() -> Vec<Route> {

	routes![
		qwiz_info,
		get_qwiz_by_id,
		create_qwiz,
		update_qwiz,
		delete_qwiz,
	]

}



#[get("/qwiz")]
fn qwiz_info() -> &'static str {
r#"
GET /qwiz/<id> - get qwiz data by id

POST /qwiz - create a qwiz
creator_password: String - required
qwiz: {
	name: String - required
	creator_id: i32 - required
	thumbnail_uri: String - optional
} - required
questions: Vector of {
	body: String - required,
	answer1: String - required,
	answer2: String - required,
	answer3: String - optional,
	answer4: String - optional,
	correct: 1 / 2 / 3 / 4 - required,
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
"#
}


#[derive(Serialize)]
struct GetQwizData {
	id: i32,
	name: String,
	creator_id: i32,
	thumbnail: Option<GetMediaData>,
	questions: Vec<GetQuestionData>,
}
impl GetQwizData {

	async fn from_qwiz(qwiz: Qwiz) -> Result<Self, sqlx::Error> {

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
			}
		)

	}

}

#[get("/qwiz/<id>")]
async fn get_qwiz_by_id(id: i32) -> Result<Json<GetQwizData>, Status> {

	match Qwiz::get_by_id(&id).await {
		Ok(qwiz) => {
			match GetQwizData::from_qwiz(qwiz).await {
				Ok(data) => Ok(Json(data)),
				Err(e) => {

					eprintln!("{e}");
					Err(Status::InternalServerError)
					
				},
			}
		},
		Err(e) => {

			eprintln!("{e}");
			Err(Status::NotFound)
			
		},
	}

}



#[derive(Deserialize)]
struct PostQwizData {
	creator_password: String,
	qwiz: NewQwizData,
	questions: Vec<NewQuestionData>,
}

#[post("/qwiz", data = "<qwiz_data>")]
async fn create_qwiz(qwiz_data: Json<PostQwizData>) -> Result<Created<String>, Status> {
	
	let mut account = match Account::get_by_id(&qwiz_data.qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Err(Status::Unauthorized)
			
		},
	};

	match account.verify_password(&qwiz_data.creator_password).await {
		Ok(true) => {
			let qwiz = match Qwiz::from_qwiz_data(&qwiz_data.qwiz).await {
				Ok(qwiz) => qwiz,
				Err(e) => {

					eprintln!("{e}");
					return Err(Status::BadRequest)
					
				},
			};

			if Question::from_question_datas(&qwiz.id, &qwiz_data.questions).await.is_err() {

				if qwiz.delete().await.is_err() {
					return Err(Status::InternalServerError);
				}
				return Err(Status::BadRequest);

			}

			Ok(Created::new(format!("{BASE_URL}/qwiz/{}", qwiz.id)))

		},
		Ok(false) => Err(Status::Unauthorized),
		Err(e) => {

			eprintln!("{e}");
			Err(Status::InternalServerError)
			
		},
	}

}



#[derive(Deserialize)]
struct PatchQwizData {
	creator_password: String,
	new_name: Option<String>,
	new_thumbnail: Option<NewMediaData>,
}

#[patch("/qwiz/<id>", data = "<new_qwiz_data>")]
async fn update_qwiz(id: i32, new_qwiz_data: Json<PatchQwizData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	let mut qwiz = match Qwiz::get_by_id(&id).await {
		Ok(q) => q,
		Err(e) => {

			eprintln!("{e}");
			return Err(Left(Status::NotFound))
			
		},
	};

	let mut account = match Account::get_by_id(&qwiz.creator_id).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Err(Left(Status::InternalServerError))
			
		},
	};

	match account.verify_password(&new_qwiz_data.creator_password).await {
		Ok(true) => {

			if let Some(new_name) = &new_qwiz_data.new_name {
				if qwiz.update_name(new_name).await.is_err() {
					return Err(Right(BadRequest(Some("Bad name"))));
				}
			}

			if let Some(new_thumbnail) = &new_qwiz_data.new_thumbnail {
				use QwizError::*;

				match qwiz.update_thumbnail(new_thumbnail).await {
					Ok(_) => (),
					Err(Sqlx(e)) => {
						
						eprintln!("{e}");
						return Err(Left(Status::InternalServerError))

					},
					Err(Base64(_)) => return Err(Right(BadRequest(Some("Bad thumbnail base64")))),
					Err(IO(e)) => {
						
						eprintln!("{e}");
						return Err(Left(Status::InternalServerError))

					},
				}
			}

			Ok(Status::Ok)

		},
		Ok(false) => Err(Left(Status::Unauthorized)),
		Err(e) => {

			eprintln!("{e}");
			Err(Left(Status::InternalServerError))
			
		},
	}

}



#[derive(Deserialize)]
struct DeleteQwizData {
	creator_password: String,
}

#[delete("/qwiz/<id>", data = "<delete_qwiz_data>")]
async fn delete_qwiz(id: i32, delete_qwiz_data: Json<DeleteQwizData>) -> Status {

	match Qwiz::get_by_id(&id).await {
		Ok(qwiz) => {
			
			let mut account = match Account::get_by_id(&qwiz.creator_id).await {
				Ok(acc) => acc,
				Err(e) => {

					eprintln!("{e}");
					return Status::InternalServerError
					
				},
			};

			match account.verify_password(&delete_qwiz_data.creator_password).await {
				Ok(true) => {
					match qwiz.delete().await {
						Ok(_) => Status::Ok,
						Err(e) => {

							eprintln!("{e}");
							Status::InternalServerError
							
						},
					}
				},
				Ok(false) => Status::Unauthorized,
				Err(e) => {

					eprintln!("{e}");
					Status::InternalServerError
					
				},
			}

		},
		Err(e) => {

			eprintln!("{e}");
			Status::NotFound
			
		},
	}

}
