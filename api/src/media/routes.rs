use crate::{BASE_URL, account::Account};
use crate::media::Media;
use rocket::{Route, response::{status::Created, Redirect}, http::Status, Either, serde::json::Json};
use serde::Deserialize;
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		media_info,
		get_by_uuid,
		add_uri,
		upload_media,
	]

}



#[get("/media")]
fn media_info() -> &'static str {
r#"
GET /media/<uuid> - redirects to the URI, associated with the UUID

POST /media - add a URI
username - String: required
password - String: required
uri - String: required

POST /media/upload - upload base64 data
username - String: reuqired
password - String: required
media_b64 - base64 String: required

"#
}



#[get("/media/<uuid>")]
async fn get_by_uuid(uuid: Uuid) -> Result<Redirect, Status> {

	match Media::get_by_uuid(&uuid).await {
		Ok(media) => Ok(Redirect::to(media.uri)),
		Err(_) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PostMediaData {
	username: String,
	password: String,
	uri: String,
}
#[post("/media", data="<data>", rank=1)]
async fn add_uri(data: Json<PostMediaData>) -> Result<Created<String>, Status> {

	let mut account = match Account::get_by_username(&data.username).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Err(Status::Unauthorized)

		}
	};

	match account.verify_password(&data.password).await {
		Ok(true) => {
			match Media::new(&data.uri).await {
				Ok(media) => Ok(Created::new(format!("{BASE_URL}/media/{}", media.uuid))),
				Err(e) => {

					eprintln!("{e}");
					Err(Status::InternalServerError)
		
				},
			}
		},
		Ok(false) => Err(Status::Unauthorized),
		Err(e) => {

			eprintln!("{e}");
			Err(Status::InternalServerError)

		}
	}

}



#[derive(Deserialize)]
struct PostUploadMediaData {
	username: String,
	password: String,
	media_b64: String,
}
#[post("/media/upload", data="<data>", rank=2)]
async fn upload_media(data: Json<PostUploadMediaData>) -> Result<Created<String>, Status> {

	let mut account = match Account::get_by_username(&data.username).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Err(Status::Unauthorized)

		}
	};

	match account.verify_password(&data.password).await {
		Ok(true) => {
			match Media::upload(&data.media_b64).await {
				Ok(media) => Ok(Created::new(format!("{BASE_URL}/media/{}", media.uuid))),
				Err(Either::Left(e)) => {

					eprintln!("{e}");
					Err(Status::InternalServerError)

				},
				Err(Either::Right(Either::Left(e))) => {

					eprintln!("{e}");
					Err(Status::BadRequest)

				},
				Err(Either::Right(Either::Right(e))) => {

					eprintln!("{e}");
					Err(Status::InternalServerError)

				},
			}
		},
		Ok(false) => Err(Status::Unauthorized),
		Err(e) => {

			eprintln!("{e}");
			Err(Status::InternalServerError)

		}
	}

}
