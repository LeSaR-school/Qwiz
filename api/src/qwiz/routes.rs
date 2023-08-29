use crate::account::Account;
use crate::qwiz::*;
use crate::crypto::*;
use rocket::Route;
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
}
impl GetQwizData {

	fn from_qwiz(qwiz: Qwiz) -> Self {
		Self {
			uuid: qwiz.uuid,
			name: qwiz.name,
			creator_uuid: qwiz.creator_uuid,
			thumbnail_uuid: qwiz.thumbnail_uuid
		}
	}

}

#[get("/qwiz/<id>", rank = 1)]
async fn get_qwiz_by_id(id: Uuid) -> Result<Json<GetQwizData>, Status> {

	match Qwiz::get_by_id(&id).await {
		Ok(qwiz) => Ok(Json(GetQwizData::from_qwiz(qwiz))),
		Err(_) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PostQwizData {
	name: String,
	creator_uuid: Uuid,
	creator_password: String,
	thumbnail_url: Option<String>,
}

#[post("/qwiz", data = "<qwiz_data>")]
async fn new_qwiz(qwiz_data: Json<PostQwizData>) -> Result<String, Status> {
	
	let mut account = match Account::get_by_id(&qwiz_data.creator_uuid).await {
		Ok(acc) => acc,
		Err(_) => return Err(Status::Unauthorized),
	};

	match verify_password(&qwiz_data.creator_password, &mut account).await {
		Ok(verified) => {
			if verified {
				match Qwiz::new(&qwiz_data.name, &qwiz_data.creator_uuid, &qwiz_data.thumbnail_url).await {
					Ok(qwiz) => Ok(qwiz.uuid.to_string()),
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
struct PatchQwizData {
	creator_password: String,
	new_name: Option<String>,
	new_thumbnail_url: Option<String>,
}

#[patch("/qwiz/<id>", data = "<new_qwiz_data>")]
async fn update_qwiz(id: Uuid, new_qwiz_data: Json<PatchQwizData>) -> Status {

	match Qwiz::get_by_id(&id).await {
		Ok(mut qwiz) => {

			let mut account = match Account::get_by_id(&qwiz.creator_uuid).await {
				Ok(acc) => acc,
				Err(_) => return Status::InternalServerError,
			};

			match verify_password(&new_qwiz_data.creator_password, &mut account).await {
				Ok(verified) => {
					if verified {

						if let Some(new_name) = &new_qwiz_data.new_name {
							if qwiz.update_name(new_name).await.is_err() {
								return Status::BadRequest;
							}
						}

						if let Some(new_thumbnail_url) = &new_qwiz_data.new_thumbnail_url {
							if qwiz.update_thumbnail_url(new_thumbnail_url).await.is_err() {
								return Status::BadRequest;
							}
						}

						Status::Ok

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



#[derive(Deserialize)]
struct DeleteQwizData {
	creator_password: String,
}

#[delete("/qwiz/<id>", data = "<delete_qwiz_data>")]
async fn delete_qwiz(id: Uuid, delete_qwiz_data: Json<DeleteQwizData>) -> Status {

	match Qwiz::get_by_id(&id).await {
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
