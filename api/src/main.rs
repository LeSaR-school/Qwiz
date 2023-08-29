mod account;
mod crypto;
mod qwiz;

#[macro_use] extern crate rocket;
extern crate sqlx;
extern crate tokio;

use crate::account::Account;

use std::str::FromStr;
use rocket::{
	serde::json::{Json},
	http::Status,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::{PgPoolOptions, PgConnectOptions}, ConnectOptions};
use uuid::Uuid;



#[launch]
fn rocket() -> _ {

	rocket::build().mount("/api", routes![
		index,
		get_account, new_account, update_account, delete_account,
	])

}



#[get("/")]
fn index() -> &'static str {
	"This is the base api page of the backend"
}



#[derive(Serialize)]
struct GetAccountData {
	username: String,
	profile_picture_uuid: Option<Uuid>,
}
impl GetAccountData {

	fn from_account(account: Account) -> Self {
		Self { username: account.username, profile_picture_uuid: account.profile_picture_uuid }
	}

}

#[get("/account/<id>")]
async fn get_account(id: Uuid) -> Result<Json<GetAccountData>, Status> {

	let pool = &PgPoolOptions::new()
	.connect_with(
		PgConnectOptions::from_str(dotenv::var("DATABASE_URL").unwrap().as_str())
			.unwrap()
			.disable_statement_logging()
			.clone()
	).await.unwrap();

	match Account::get_by_id(id, pool).await {
		Ok(account) => Ok(Json(GetAccountData::from_account(account))),
		Err(_) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PostAccountData {
	username: String,
	password: String,
}

#[post("/account", data = "<account_data>")]
async fn new_account(account_data: Json<PostAccountData>) -> Result<String, Status> {
	
	let pool = &PgPoolOptions::new()
	.connect_with(
		PgConnectOptions::from_str(dotenv::var("DATABASE_URL").unwrap().as_str())
			.unwrap()
			.disable_statement_logging()
			.clone()
	).await.unwrap();

	match Account::new(&account_data.username, &account_data.password, pool).await {
		Ok(account) => Ok(account.uuid.to_string()),
		Err(_) => Err(Status::BadRequest),
	}

}



#[derive(Deserialize)]
struct PatchAccountData {
	old_password: String,
	new_password: String,
}

#[patch("/account/<id>", data = "<new_account_data>")]
async fn update_account(id: Uuid, new_account_data: Json<PatchAccountData>) -> Status {

	let pool = &PgPoolOptions::new()
	.connect_with(
		PgConnectOptions::from_str(dotenv::var("DATABASE_URL").unwrap().as_str())
			.unwrap()
			.disable_statement_logging()
			.clone()
	).await.unwrap();

	match Account::get_by_id(id, pool).await {
		Ok(mut account) => {
			if crypto::verify_password(&new_account_data.old_password, &account.password_hash) {
				match account.update_password(&new_account_data.new_password, pool).await {
					Ok(_) => Status::Ok,
					Err(_) => Status::BadRequest,
				}
			} else {
				Status::Unauthorized
			}
		},
		Err(_) => Status::NotFound,
	}

}



#[derive(Deserialize)]
struct DeleteAccountData {
	password: String,
}

#[delete("/account/<id>", data = "<delete_account_data>")]
async fn delete_account(id: Uuid, delete_account_data: Json<DeleteAccountData>) -> Status {

	let pool = &PgPoolOptions::new()
	.connect_with(
		PgConnectOptions::from_str(dotenv::var("DATABASE_URL").unwrap().as_str())
			.unwrap()
			.disable_statement_logging()
			.clone()
	).await.unwrap();

	match Account::get_by_id(id, pool).await {
		Ok(account) => {
			if crypto::verify_password(&delete_account_data.password, &account.password_hash) {
				match account.delete(pool).await {
					Ok(_) => Status::Ok,
					Err(_) => Status::InternalServerError,
				}
			} else {
				Status::Unauthorized
			}
		},
		Err(_) => Status::NotFound,
	}

}
