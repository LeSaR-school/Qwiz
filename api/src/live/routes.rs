use crate::account::Account;
use crate::{BASE_URL, log_err, db_err_to_status, internal_err};
use crate::live::{LiveQwiz, LiveQwizState, LiveQwizParticipant, LiveQwizError};
use crate::media::{Media, MediaType};
use crate::question::Question;

use rocket::Route;
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};



pub fn all() -> Vec<Route> {

	routes![
		state,
		create,
		add_participants,
		remove_participants,
	]

}




#[derive(Deserialize)]
pub struct LiveQwizOptions {
	pub shuffle_questions: bool,
	pub shuffle_answers: bool,
}

#[derive(Serialize)]
struct MediaData {
	uri: String,
	r#type: MediaType,
}

#[derive(Serialize)]
struct QuestionData {
	body: String,
	embed: Option<MediaData>,
	answers: Vec<String>,
}
impl QuestionData {
	async fn from_question(question: &Question) -> Self {

		let mut answers = vec![question.answer1.to_owned(), question.answer2.to_owned()];
		if let Some(a3) = &question.answer3 {
			answers.push(a3.to_owned());
		}
		if let Some(a4) = &question.answer4 {
			answers.push(a4.to_owned());
		}

		let media = match question.embed_uuid {
			Some(uuid) => Media::get_by_uuid(&uuid).await.map_err(|e| log_err(&e)).ok(),
			None => None,
		};

		Self {
			body: question.body.to_owned(),
			embed: media.map(|m| MediaData { uri: m.uri, r#type: m.media_type }),
			answers,
		}

	}
}

#[derive(Serialize)]
pub struct GetLiveQwizData {
	qwiz_id: i32,
	host_id: i32,
	participants: Vec<LiveQwizParticipant>,
	state: LiveQwizState,
	current_question: Option<QuestionData>,
}
impl GetLiveQwizData {
	pub async fn from_live_qwiz(live_qwiz: &LiveQwiz) -> Self {

		use LiveQwizState::*;

		GetLiveQwizData {
			qwiz_id: live_qwiz.qwiz_id,
			host_id: live_qwiz.host_id,
			participants: live_qwiz.participants.clone(),
			state: live_qwiz.state.clone(),
			current_question: match live_qwiz.state {
				Starting | Finished => None,
				Live(current_question) => match live_qwiz.questions.get(current_question) {
					Some(q) => Some(QuestionData::from_question(q).await),
					None => None,
				},
			}
		}

	}
}

#[get("/live/<id>")]
async fn state(id: u16) -> Result<Json<GetLiveQwizData>, Status> {

	match LiveQwiz::get_data(&id).await {
		Some(data) => Ok(Json(data)),
		None => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PostLiveQwizData {
	username: String,
	password: String,
	qwiz_id: i32,
	options: LiveQwizOptions,
}

#[post("/live", data = "<live_qwiz_data>")]
async fn create(live_qwiz_data: Json<PostLiveQwizData>) -> Result<Created<String>, Status> {

	let mut host = match Account::get_by_username(&live_qwiz_data.username).await {
		Ok(acc) => acc,
		Err(e) => return Err(db_err_to_status(&e, Status::Unauthorized)),
	};

	match host.verify_password(&live_qwiz_data.password).await {
		Ok(true) => (),
		Ok(false) => return Err(Status::Unauthorized),
		Err(e) => return Err(internal_err(&e)),
	}

	match LiveQwiz::new(host.id, live_qwiz_data.qwiz_id, &live_qwiz_data.options).await {
		Ok(lq) => Ok(Created::new(format!("{BASE_URL}/live/{}", lq.id))),
		Err(e) => Err(internal_err(&e)),
	}

}



#[derive(Deserialize)]
pub struct PutLiveQwizParticipant {
	pub id: i32,
	pub display_name: Option<String>,
}

#[derive(Deserialize)]
struct PutParticipantsData {
	username: String,
	password: String,
	participants: Vec<PutLiveQwizParticipant>,
}

#[put("/live/<id>", data = "<put_participants_data>")]
async fn add_participants(id: u16, put_participants_data: Json<PutParticipantsData>) -> Status {

	use LiveQwizError::*;

	let host_id = match LiveQwiz::get_data(&id).await {
		Some(lq) => lq.host_id,
		None => return Status::NotFound,
	};

	let mut host = match Account::get_by_username(&put_participants_data.username).await {
		Ok(acc) => acc,
		Err(e) => return db_err_to_status(&e, Status::Unauthorized),
	};

	if host.id != host_id {
		return Status::Unauthorized;
	}

	match host.verify_password(&put_participants_data.password).await {
		Ok(true) => (),
		Ok(false) => return Status::Unauthorized,
		Err(e) => return internal_err(&e),
	}

	match LiveQwiz::add_participants(&id, &put_participants_data.participants).await {
		Ok(_) => Status::Ok,
		Err(QwizNotFound(_)) => Status::NotFound,
		Err(Sqlx(e)) => internal_err(&e),
	}

}



#[derive(Deserialize)]
struct DeleteParticipantsData {
	username: String,
	password: String,
	participant_ids: Vec<i32>,
}

#[delete("/live/<id>", data = "<delete_participants_data>")]
async fn remove_participants(id: u16, delete_participants_data: Json<DeleteParticipantsData>) -> Status {
	
	use LiveQwizError::*;

	let host_id = match LiveQwiz::get_data(&id).await {
		Some(lq) => lq.host_id,
		None => return Status::NotFound,
	};

	let mut host = match Account::get_by_username(&delete_participants_data.username).await {
		Ok(acc) => acc,
		Err(e) => return db_err_to_status(&e, Status::Unauthorized),
	};

	if host.id != host_id {
		return Status::Unauthorized;
	}

	match host.verify_password(&delete_participants_data.password).await {
		Ok(true) => (),
		Ok(false) => return Status::Unauthorized,
		Err(e) => return internal_err(&e),
	}

	match LiveQwiz::remove_participants(&id, &delete_participants_data.participant_ids).await {
		Ok(_) => Status::Ok,
		Err(QwizNotFound(_)) => Status::NotFound,
		Err(Sqlx(e)) => internal_err(&e),
	}

}
