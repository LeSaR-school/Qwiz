use crate::account::Account;
use crate::question::routes::PostQuestionData;
use crate::qwiz::Qwiz;
use crate::question::Question;
use crate::crypto::verify_password;

use rocket::response::status::{BadRequest, Created};
use rocket::{Route, Either};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		qwiz_info,
		get_qwiz_by_id,
		new_qwiz,
		update_qwiz,
		delete_qwiz,
	]

}



#[get("/qwiz")]
fn qwiz_info() -> &'static str {
r#"
GET /qwiz/<uuid> - get qwiz data by uuid

POST /qwiz - create an qwiz using json
"name": String - required
"creator_uuid": Uuid - required
"creator_password": String - required
"thumbnail_url": String - optional

PATCH /qwiz/<uuid> - update qwiz data using json
"creator_password": String - required
"new_name": String - optional
"new_thumbnail_url": String - optional

DELETE /qwiz/<uuid> - delete qwiz using json
"creator_password": String - required

"#
}


#[derive(Serialize)]
struct GetQwizData {
	uuid: Uuid,
	name: String,
	creator_uuid: Uuid,
	thumbnail_uuid: Option<Uuid>,
	questions: Vec<Question>,
}
impl GetQwizData {

	async fn from_qwiz(qwiz: Qwiz) -> Result<Self, sqlx::Error> {
		Ok(
			Self {
				uuid: qwiz.uuid,
				name: qwiz.name,
				creator_uuid: qwiz.creator_uuid,
				thumbnail_uuid: qwiz.thumbnail_uuid,
				questions: Question::get_all_by_qwiz_uuid(&qwiz.uuid).await?,
			}
		)
	}

}

#[get("/qwiz/<uuid>", rank = 1)]
async fn get_qwiz_by_id(uuid: Uuid) -> Result<Json<GetQwizData>, Status> {

	match Qwiz::get_by_uuid(&uuid).await {
		Ok(qwiz) => Ok(Json(
			match GetQwizData::from_qwiz(qwiz).await {
				Ok(data) => data,
				Err(_) => return Err(Status::InternalServerError),
			}
		)),
		Err(_) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PostQwizData {
	name: String,
	creator_uuid: Uuid,
	creator_password: String,
	thumbnail_url: Option<String>,
	questions: Option<Vec<PostQuestionData>>,
}

#[post("/qwiz", data = "<qwiz_data>")]
async fn new_qwiz(qwiz_data: Json<PostQwizData>) -> Result<Created<String>, Status> {
	
	let mut account = match Account::get_by_id(&qwiz_data.creator_uuid).await {
		Ok(acc) => acc,
		Err(_) => return Err(Status::Unauthorized),
	};

	match verify_password(&qwiz_data.creator_password, &mut account).await {
		Ok(verified) => {
			if verified {

				let qwiz_uuid = match Qwiz::new(&qwiz_data.name, &qwiz_data.creator_uuid, &qwiz_data.thumbnail_url).await {
					Ok(qwiz) => qwiz.uuid,
					Err(_) => return Err(Status::BadRequest),
				};

				if let Some(questions) = &qwiz_data.questions {
					for question_data in questions {
						if Question::from_question_data(&qwiz_uuid, &question_data).await.is_err() {
							return Err(Status::BadRequest);
						}
					}
				}

				Ok(Created::new(qwiz_uuid.to_string()))

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

#[patch("/qwiz/<uuid>", data = "<new_qwiz_data>")]
async fn update_qwiz(uuid: Uuid, new_qwiz_data: Json<PatchQwizData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	match Qwiz::get_by_uuid(&uuid).await {
		Ok(mut qwiz) => {

			let mut account = match Account::get_by_id(&qwiz.creator_uuid).await {
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

#[delete("/qwiz/<uuid>", data = "<delete_qwiz_data>")]
async fn delete_qwiz(uuid: Uuid, delete_qwiz_data: Json<DeleteQwizData>) -> Status {

	match Qwiz::get_by_uuid(&uuid).await {
		Ok(qwiz) => {
			
			let mut account = match Account::get_by_id(&qwiz.creator_uuid).await {
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
