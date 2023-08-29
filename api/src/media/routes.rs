use crate::media::{Media, MediaType};
use rocket::{
	Route,
	http::Status,
	serde::json::Json
};
use serde::Serialize;
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		media_info,
		get_media_by_uuid,
		// post_media,
	]

}



#[get("/media")]
fn media_info() -> &'static str {
r#"
GET /media/<uuid> - get media data by uuid
"#

}



#[derive(Serialize)]
pub struct GetMediaData {
	uri: String,
	media_type: MediaType
}
impl From<Media> for GetMediaData {

	fn from(value: Media) -> Self {
		Self {
			uri: value.uri,
			media_type: value.media_type,
		}
	}

}

#[get("/media/<uuid>")]
async fn get_media_by_uuid(uuid: Uuid) -> Result<Json<GetMediaData>, Status> {

	match Media::get_by_uuid(&uuid).await {
		Ok(media) => Ok(Json(media.into())),
		Err(e) => {
			
			eprintln!("{e}");
			Err(Status::NotFound)
			
		},
	}

}
