use crate::media::MediaError;
use crate::{BASE_URL, internal_err, db_err_to_status};
use crate::account::{Account, AccountType, AccountError, NewAccountError};
use crate::media::{Media, NewMediaData, routes::GetMediaData};
use rocket::response::status::{BadRequest, Created};
use rocket::{Route, Either::{self, *}};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};



pub fn all() -> Vec<Route> {

	routes![
		account_info,
		get_account_by_id,
		get_account_by_username,
		verify_password,
		create_account,
		update_account,
		delete_account,
	]

}



#[get("/account")]
fn account_info() -> &'static str {
r#"
enum AccountType ( "Student", "Parent", "Teacher" )

GET /account/<id> - get account data by id
GET /account/<username> - get account data by username

POST /account - create an account
username: String - required
password: String - required
account_type: AccountType - required
profile_picture: {
	data: String - required
	media_type: MediaType - required
} - optional

PATCH /account/<id> - update account data
password: String - required
new_password: String - optional
new_account_type: AccountType - optional
new_profile_picture: {
	data: String - required
	media_type: MediaType - required
} - optional

DELETE /account/<id> - delete account
password: String - required

GET /account/<id>/classes - get student/teacher account classes
password: String - required

GET /account/<id>/assignments - get student assignments
password: String - required
"#
}



#[derive(Serialize)]
struct GetAccountData {
	id: i32,
	username: String,
	profile_picture: Option<GetMediaData>,
	account_type: AccountType,
}
impl GetAccountData {

	async fn from_account(account: Account) -> Self {
		Self {
			id: account.id,
			username: account.username,
			profile_picture: match account.profile_picture_uuid {
				Some(uuid) => Media::get_by_uuid(&uuid).await.ok().map(Into::into),
				None => None,
			},
			account_type: account.account_type
		}
	}

}

#[get("/account/<id>", rank = 1)]
async fn get_account_by_id(id: i32) -> Result<Json<GetAccountData>, Status> {

	match Account::get_by_id(&id).await {
		Ok(account) => Ok(Json(GetAccountData::from_account(account).await)),
		Err(e) => Err(db_err_to_status(&e, Status::NotFound)),
	}
	
}

#[get("/account/<username>", rank = 2)]
async fn get_account_by_username(username: String) -> Result<Json<GetAccountData>, Status> {

	match Account::get_by_username(&username).await {
		Ok(account) => Ok(Json(GetAccountData::from_account(account).await)),
		Err(e) => Err(db_err_to_status(&e, Status::NotFound)),
	}

}



#[derive(Deserialize)]
struct VerifyPasswordData {
	username: String,
	password: String,
}

#[post("/account/verify", data = "<account_data>")]
async fn verify_password(account_data: Json<VerifyPasswordData>) -> Status {
	match Account::get_by_username(&account_data.username).await {
		Ok(mut account) => {
			match account.verify_password(&account_data.password).await {
				Ok(true) => Status::Ok,
				Ok(false) => Status::Unauthorized,
				Err(e) => internal_err(&e),
			}
		},
		Err(e) => db_err_to_status(&e, Status::NotFound),
	}
}

#[derive(Deserialize)]
struct PostAccountData {
	username: String,
	password: String,
	account_type: AccountType,
	profile_picture: Option<NewMediaData>,
}

#[post("/account", data = "<account_data>")]
async fn create_account(account_data: Json<PostAccountData>) -> Result<Created<String>, Either<Status, BadRequest<String>>> {

	use NewAccountError::*;
	use AccountError::*;

	match Account::new(&account_data.username, &account_data.password, &account_data.account_type, &account_data.profile_picture).await {
		Ok(account) => Ok(Created::new(format!("{BASE_URL}/account/{}", account.id))),
		Err(Base(Sqlx(e))) => Err(Left(internal_err(&e))),
		Err(Base(IO(e))) => Err(Left(internal_err(&e))),
		Err(e) => Err(Right(BadRequest(Some(e.to_string())))),
	}

}



#[derive(Deserialize)]
struct PatchAccountData {
	password: String,
	new_password: Option<String>,
	new_account_type: Option<AccountType>,
	new_profile_picture: Option<NewMediaData>,
}

#[patch("/account/<id>", data = "<new_account_data>")]
async fn update_account(id: i32, new_account_data: Json<PatchAccountData>) -> Result<Status, Either<Status, BadRequest<&'static str>>> {

	
	let mut account = match Account::get_by_id(&id).await {
		Ok(acc) => acc,
		Err(e) => return Err(Left(db_err_to_status(&e, Status::NotFound))),
	};
	
	match account.verify_password(&new_account_data.password).await {
		Ok(true) => (),
		Ok(false) => return Err(Left(Status::Unauthorized)),
		Err(e) => return Err(Left(internal_err(&e))),
	}


	
	if let Some(new_account_type) = &new_account_data.new_account_type {
		if account.update_account_type(new_account_type).await.is_err() {
			return Err(Right(BadRequest(Some("bad account type"))));
		}
	}
	
	if let Some(new_password) = &new_account_data.new_password {
		match account.update_password(new_password).await {
			Ok(true) => (),
			Ok(false) => return Err(Right(BadRequest(Some("bad password")))),
			Err(e) => return Err(Left(internal_err(&e))),
		}
	}
	
	if let Some(new_profile_picture) = &new_account_data.new_profile_picture {
		use MediaError::*;
		
		match account.update_profile_picture(new_profile_picture).await {
			Ok(_) => (),
			Err(Sqlx(e)) => return Err(Left(internal_err(&e))),
			Err(Base64(_)) => return Err(Right(BadRequest(Some("bad profile picture base64")))),
			Err(IO(e)) => return Err(Left(internal_err(&e))),
		}
	}

	Ok(Status::Ok)

}



#[derive(Deserialize)]
struct DeleteAccountData {
	password: String,
}

#[delete("/account/<id>", data = "<delete_account_data>")]
async fn delete_account(id: i32, delete_account_data: Json<DeleteAccountData>) -> Status {

	if !Account::exists_id(&id).await {
		return Status::NotFound
	}

	let mut account = match Account::get_by_id(&id).await {
		Ok(acc) => acc,
		Err(e) => return internal_err(&e),
	};

	match account.verify_password(&delete_account_data.password).await {
		Ok(true) => (),
		Ok(false) => return Status::Unauthorized,
		Err(e) => return internal_err(&e),
	}



	match account.delete().await {
		Ok(_) => Status::Ok,
		Err(e) => internal_err(&e),
	}

}
