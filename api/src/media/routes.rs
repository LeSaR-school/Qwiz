use crate::media;

use rocket::{http::Status, Route};
use uuid::Uuid;

pub fn all() -> Vec<Route> {

	routes![
		media_info,
		get_media,
	]

}

#[get("/media")]
fn media_info() -> &'static str {
r#"
GET /media/<uuid> - get media url by uuid
"#
}

#[get("/media/<uuid>")]
async fn get_media(uuid: Uuid) -> Result<String, Status> {

	match media::get(&uuid).await {
		Ok(url) => Ok(url),
		Err(_) => Err(Status::NotFound),
	}

}