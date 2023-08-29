use crate::account::*;
use crate::crypto::*;
use rocket::Route;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;



pub fn all() -> Vec<Route> {

	routes![
		account_info,
		get_account_by_id,
		get_account_by_username,
		new_account,
		update_account,
		delete_account,
	]

}



#[get("/account")]
fn account_info() -> &'static str {
r#"
enum AccountType { "Student", "Parent", "Teacher" }

GET /account/<uuid> - get account data by uuid
GET /account/<username> - get account data by username

POST /account - create an account using json
"username": String - required
"password": String - required
"account_type": AccountType - required
"profile_picture_uuid": Uuid - optional

PATCH /account/<uuid> - update account data using json
"password": String - required
"new_password": String - optional
"new_account_type": AccountType - optional

DELETE /account/<uuid> - delete account using json
"password": String - required

"#
}



#[derive(Serialize)]
struct GetAccountData {
	uuid: Uuid,
	username: String,
	profile_picture_uuid: Option<Uuid>,
	account_type: AccountType,
}
impl GetAccountData {

	fn from_account(account: Account) -> Self {
		Self {
			uuid: account.uuid,
			username: account.username,
			profile_picture_uuid: account.profile_picture_uuid,
			account_type: account.account_type
		}
	}

}

#[get("/account/<id>", rank = 1)]
async fn get_account_by_id(id: Uuid) -> Result<Json<GetAccountData>, Status> {

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
}

#[post("/account", data = "<account_data>")]
async fn new_account(account_data: Json<PostAccountData>) -> Result<String, Status> {

	match Account::new(&account_data.username, &account_data.password, &account_data.account_type).await {
		Ok(account) => Ok(account.uuid.to_string()),
		Err(_) => Err(Status::Conflict),
	}

}



#[derive(Deserialize)]
struct PatchAccountData {
	password: String,
	new_password: Option<String>,
	new_account_type: Option<AccountType>,
}

#[patch("/account/<id>", data = "<new_account_data>")]
async fn update_account(id: Uuid, new_account_data: Json<PatchAccountData>) -> Status {

	match Account::get_by_id(&id).await {
		Ok(mut account) => {
			match verify_password(&new_account_data.password, &mut account).await {
				Ok(verified) => {
					if verified {

						if let Some(new_account_type) = &new_account_data.new_account_type {
							if account.update_account_type(new_account_type).await.is_err() {
								return Status::BadRequest;
							}
						}

						if let Some(new_password) = &new_account_data.new_password {
							if account.update_password(new_password).await.is_err() {
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
struct DeleteAccountData {
	password: String,
}

#[delete("/account/<id>", data = "<delete_account_data>")]
async fn delete_account(id: Uuid, delete_account_data: Json<DeleteAccountData>) -> Status {

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
