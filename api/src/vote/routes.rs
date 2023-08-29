use crate::account::Account;
use crate::vote::{Vote, VoteData, VoteError};
use rocket::{Route, http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};



pub fn all() -> Vec<Route> {

	routes![
		vote_info,
		get_votes,
		add_vote,
		delete_vote,
	]

}



#[get("/vote")]
fn vote_info() -> &'static str {
r#"
GET /vote/<qwiz_id> - get list of voter ids by qwiz id

PUT /vote/<qwiz_id> - vote for a qwiz
voter_password: String - required
vote: {
	voter_id: i32 - required,
	qwiz_id: i32 - required
} - required

DELETE /vote/<qwiz_id> - delete vote
voter_password: String - required
vote: {
	voter_id: i32 - required,
	qwiz_id: i32 - required
} - required
"#
}



#[derive(Serialize)]
struct GetVoteData {
	voter_ids: Vec<i32>,
}
impl GetVoteData {
	fn from_votes(votes: Vec<Vote>) -> Self {
		Self {
			voter_ids: votes.iter().map(|vote| vote.voter_id).collect(),
		}
	}
}

#[get("/vote/<qwiz_id>")]
async fn get_votes(qwiz_id: i32) -> Result<Json<GetVoteData>, Status> {

	use VoteError::*;

	match Vote::get_all_by_qwiz_id(&qwiz_id).await {
		Ok(votes) => Ok(Json(GetVoteData::from_votes(votes))),
		Err(Sqlx(e)) => {

			eprintln!("{e}");
			Err(Status::InternalServerError)

		},
		Err(QwizNotFound) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PutVoteData {
	voter_password: String,
	vote: VoteData,
}

#[put("/vote/<qwiz_id>", data = "<vote_data>")]
async fn add_vote(qwiz_id: i32, vote_data: Json<PutVoteData>) -> Status {

	use VoteError::*;

	let mut account = match Account::get_by_id(&vote_data.vote.voter_id).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Status::Unauthorized
			
		},
	};

	match account.verify_password(&vote_data.voter_password).await {
		Ok(true) => {
			match Vote::exists(&vote_data.vote.voter_id, &qwiz_id).await {
				Ok(true) => Status::NoContent,
				Ok(false) => {
					match Vote::from_vote_data(&vote_data.vote).await {
						Ok(_) => Status::Ok,
						Err(Sqlx(e)) => {

							eprintln!("{e}");
							Status::InternalServerError

						},
						Err(QwizNotFound) => Status::NotFound,
					}
				},
				Err(Sqlx(e)) => {

					eprintln!("{e}");
					Status::InternalServerError

				},
				Err(QwizNotFound) => Status::NotFound,
			}
		},
		Ok(false) => Status::Unauthorized,
		Err(e) => {

			eprintln!("{e}");
			Status::InternalServerError

		},
	}

}



#[derive(Deserialize)]
struct DeleteVoteData {
	voter_password: String,
	vote: VoteData,
}

#[delete("/vote/<qwiz_id>", data = "<delete_vote_data>")]
async fn delete_vote(qwiz_id: i32, delete_vote_data: Json<DeleteVoteData>) -> Status {

	use VoteError::*;

	let mut account = match Account::get_by_id(&delete_vote_data.vote.voter_id).await {
		Ok(acc) => acc,
		Err(e) => {

			eprintln!("{e}");
			return Status::Unauthorized
			
		},
	};

	match account.verify_password(&delete_vote_data.voter_password).await {
		Ok(true) => {
			match Vote::get_by_voter_id_qwiz_id(&delete_vote_data.vote.voter_id, &qwiz_id).await {
				Ok(None) => Status::NoContent,
				Ok(Some(vote)) => {
					match vote.delete().await {
						Ok(_) => Status::Ok,
						Err(e) => {

							eprintln!("{e}");
							Status::InternalServerError

						},
					}
				},
				Err(Sqlx(e)) => {

					eprintln!("{e}");
					Status::InternalServerError

				},
				Err(QwizNotFound) => Status::NotFound,
			}
		},
		Ok(false) => Status::Unauthorized,
		Err(e) => {

			eprintln!("{e}");
			Status::InternalServerError

		},
	}

}