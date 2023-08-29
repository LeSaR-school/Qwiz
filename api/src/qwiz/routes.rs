use crate::BASE_URL;
use crate::account::Account;
use crate::qwiz::Qwiz;
use crate::question::{Question, NewQuestionData, routes::GetQuestionData};
use crate::media;
use crate::crypto::verify_password;

use rocket::response::status::{BadRequest, Created};
use rocket::{Route, Either};
use rocket::{http::Status, serde::json::Json};
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
"creator_password": String - required
"qwiz": {
	"name": String - required
	"creator_id": i32 - required
	"thumbnail_url": String - optional
} - required
"questions": Vector of {
	"body": String - required,
	"answer1": String - required,
	"answer2": String - required,
	"answer3": String - optional,
	"answer4": String - optional,
	"correct": 1 / 2 / 3 / 4 - required,
	"embed_url": String - optional,
} - required

PATCH /qwiz/<id> - update qwiz data
"creator_password": String - required
"new_name": String - optional
"new_thumbnail_url": String - optional

DELETE /qwiz/<id> - delete qwiz
"creator_password": String - required

"#
}


#[derive(Serialize)]
struct GetQwizData {
	id: i32,
	name: String,
	creator_id: i32,
	thumbnail_url: Option<String>,
	questions: Vec<GetQuestionData>,
}
impl GetQwizData {

	async fn from_qwiz(qwiz: Qwiz) -> Result<Self, sqlx::Error> {

		let mut questions: Vec<GetQuestionData> = Vec::new();
		for question in Question::get_all_by_qwiz_id(&qwiz.id).await? {
			questions.push(GetQuestionData::from_question(question).await?);
		}

		Ok(
			Self {
				id: qwiz.id,
				name: qwiz.name,
				creator_id: qwiz.creator_id,
				thumbnail_url: match qwiz.thumbnail_uuid {
					Some(id) => media::get_by_uuid(&id).await.ok(),
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
				Err(_) => Err(Status::InternalServerError),
			}
		},
		Err(_) => Err(Status::NotFound),
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
		Err(_) => return Err(Status::Unauthorized),
	};

	match verify_password(&qwiz_data.creator_password, &mut account).await {
		Ok(verified) => {
			if verified {

				let qwiz = match Qwiz::from_qwiz_data(&qwiz_data.qwiz).await {
					Ok(qwiz) => qwiz,
					Err(_) => return Err(Status::BadRequest),
				};

				if Question::from_question_datas(&qwiz.id, &qwiz_data.questions).await.is_err() {

					if qwiz.delete().await.is_err() {
						return Err(Status::InternalServerError);
					}
					return Err(Status::BadRequest);

				}

				Ok(Created::new(format!("{}/qwiz/{}", BASE_URL, qwiz.id.to_string())))

			} else {
				Err(Status::Unauthorized)
			}
		},
		Err(_) => Err(Status::InternalServerError),
	}

}



#[derive(Deserialize)]
struct PatchQwizData {
	creator_password: String,
	new_name: Option<String>,
	new_thumbnail_url: Option<String>,
}

#[patch("/qwiz/<id>", data = "<new_qwiz_data>")]
async fn update_qwiz(id: i32, new_qwiz_data: Json<PatchQwizData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	match Qwiz::get_by_id(&id).await {
		Ok(mut qwiz) => {

			let mut account = match Account::get_by_id(&qwiz.creator_id).await {
				Ok(acc) => acc,
				Err(_) => return Err(Either::Left(Status::InternalServerError)),
			};

			match verify_password(&new_qwiz_data.creator_password, &mut account).await {
				Ok(verified) => {
					if verified {

						if let Some(new_name) = &new_qwiz_data.new_name {
							if qwiz.update_name(new_name).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad name"))));
							}
						}

						if let Some(new_thumbnail_url) = &new_qwiz_data.new_thumbnail_url {
							if qwiz.update_thumbnail_url(new_thumbnail_url).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad thumbnail url"))));
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
struct DeleteQwizData {
	creator_password: String,
}

#[delete("/qwiz/<id>", data = "<delete_qwiz_data>")]
async fn delete_qwiz(id: i32, delete_qwiz_data: Json<DeleteQwizData>) -> Status {

	match Qwiz::get_by_id(&id).await {
		Ok(qwiz) => {
			
			let mut account = match Account::get_by_id(&qwiz.creator_id).await {
				Ok(acc) => acc,
				Err(_) => return Status::InternalServerError,
			};

			match verify_password(&delete_qwiz_data.creator_password, &mut account).await {
				Ok(verified) => {
					if verified {
						match qwiz.delete().await {
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
