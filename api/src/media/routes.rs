use crate::{media::{Media, MediaType}, db_err_to_status};
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
	]

}



#[get("/media")]
fn media_info() -> &'static str {	
r#"
enum MediaType ( "Image", "Video", "Audio", "Youtube", "Gif" )

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
		Err(e) => Err(db_err_to_status(&e, Status::NotFound)),
	}

}
