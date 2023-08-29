use crate::BASE_URL;
use crate::account::{Account, AccountType};
use crate::crypto::verify_password;
use rocket::response::status::{BadRequest, Created};
use rocket::{Route, Either};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		account_info,
		get_account_by_id,
		get_account_by_username,
		create_account,
		update_account,
		delete_account,
	]

}



#[get("/account")]
fn account_info() -> &'static str {
r#"
enum AccountType { "Student", "Parent", "Teacher" }

GET /account/<id> - get account data by id
GET /account/<username> - get account data by username

POST /account - create an account
"username": String - required
"password": String - required
"account_type": AccountType - required
"profile_picture_url": String - optional

PATCH /account/<id> - update account data
"password": String - required
"new_password": String - optional
"new_account_type": AccountType - optional
"new_profile_picture_url": String - optional

DELETE /account/<id> - delete account
"password": String - required
"#
}



#[derive(Serialize)]
struct GetAccountData {
	id: i32,
	username: String,
	profile_picture_uuid: Option<Uuid>,
	account_type: AccountType,
}
impl GetAccountData {

	fn from_account(account: Account) -> Self {
		Self {
			id: account.id,
			username: account.username,
			profile_picture_uuid: account.profile_picture_uuid,
			account_type: account.account_type
		}
	}

}

#[get("/account/<id>", rank = 1)]
async fn get_account_by_id(id: i32) -> Result<Json<GetAccountData>, Status> {

	match Account::get_by_id(&id).await {
		Ok(account) => Ok(Json(GetAccountData::from_account(account))),
		Err(_) => Err(Status::NotFound),
	}
	
}

#[get("/account/<username>", rank = 2)]
async fn get_account_by_username(username: String) -> Result<Json<GetAccountData>, Status> {

	match Account::get_by_username(&username).await {
		Ok(account) => Ok(Json(GetAccountData::from_account(account))),
		Err(_) => Err(Status::NotFound),
	}

}



#[derive(Deserialize)]
struct PostAccountData {
	username: String,
	password: String,
	account_type: AccountType,
	profile_picture_url: Option<String>,
}

#[post("/account", data = "<account_data>")]
async fn create_account(account_data: Json<PostAccountData>) -> Result<Created<String>, Status> {

	match Account::new(&account_data.username, &account_data.password, &account_data.account_type, &account_data.profile_picture_url).await {
		Ok(account) => Ok(Created::new(format!("{}/account/{}", BASE_URL, account.id.to_string()))),
		Err(_) => Err(Status::Conflict),
	}

}



#[derive(Deserialize)]
struct PatchAccountData {
	password: String,
	new_password: Option<String>,
	new_account_type: Option<AccountType>,
	new_profile_picture_url: Option<String>,
}

#[patch("/account/<id>", data = "<new_account_data>")]
async fn update_account(id: i32, new_account_data: Json<PatchAccountData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	match Account::get_by_id(&id).await {
		Ok(mut account) => {
			match verify_password(&new_account_data.password, &mut account).await {
				Ok(verified) => {
					if verified {

						if let Some(new_account_type) = &new_account_data.new_account_type {
							if account.update_account_type(new_account_type).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad account type"))));
							}
						}

						if let Some(new_password) = &new_account_data.new_password {
							if account.update_password(new_password).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad password"))));
							}
						}

						if let Some(new_profile_picture_url) = &new_account_data.new_profile_picture_url {
							if account.update_profile_picture_url(new_profile_picture_url).await.is_err() {
								return Err(Either::Right(BadRequest(Some("Bad profile picture url"))));
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
struct DeleteAccountData {
	password: String,
}

#[delete("/account/<id>", data = "<delete_account_data>")]
async fn delete_account(id: i32, delete_account_data: Json<DeleteAccountData>) -> Status {

	match Account::get_by_id(&id).await {
		Ok(mut account) => {
			match verify_password(&delete_account_data.password, &mut account).await {
				Ok(verified) => {
					if verified {
						match account.delete().await {
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
