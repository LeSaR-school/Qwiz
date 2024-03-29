mod account;
mod qwiz;
mod question;
mod vote;
mod class;
mod assignment;
mod media;
mod crypto;



#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;



use crate::account::Account;
use core::fmt;
use std::{str::FromStr, env::var, ops::Deref};
use rocket::{http::Status, Request, fs::{FileServer, relative}};
use serde::Serialize;
use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgConnectOptions}, ConnectOptions};
use async_once::AsyncOnce;
use dotenv::dotenv;



pub static BASE_URL: &str = "/api";
lazy_static! {

	pub static ref POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {

		dotenv().ok();
		
		PgPoolOptions::new()
			.connect_with(
				PgConnectOptions::from_str(&var("DATABASE_URL").expect("Please set DATABASE_URL environment variable")).expect("Please provide a valid database url")
				.disable_statement_logging().clone()
			)
			.await
			.unwrap()

	});

}



#[launch]
async fn rocket() -> _ {

	Account::load_cache().await.unwrap();

	let mut routes = routes![root_info];
	routes.append(&mut account::routes::all());
	routes.append(&mut qwiz::routes::all());
	routes.append(&mut question::routes::all());
	routes.append(&mut vote::routes::all());
	routes.append(&mut class::routes::all());
	routes.append(&mut assignment::routes::all());
	routes.append(&mut media::routes::all());

	rocket::build()
		.register(BASE_URL, catchers![default_catcher])
		.mount(BASE_URL, routes)
		.mount(format!("{BASE_URL}/media/upload"), FileServer::from(relative!("media")))

}



#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> String {
	status.code.to_string()
}

#[get("/")]
fn root_info() -> &'static str {
r#"
/account
/qwiz
/question
/class
/vote
/media
"#
}



pub fn log_err(e: &dyn fmt::Display) {

	eprintln!("\x1b[0;31mERROR: {e}\x1b[0m");

}

pub fn internal_err(e: &dyn fmt::Display) -> Status {

	log_err(e);
	Status::InternalServerError

}

pub fn db_err_to_status(e: &sqlx::Error, status: Status) -> Status {

	match e {
		sqlx::Error::Database(_) | sqlx::Error::RowNotFound => status,
		e => internal_err(e),
	}

}



pub struct OptBool(pub bool);
impl From<bool> for OptBool {
	fn from(value: bool) -> Self {
		Self(value)
	}
}
impl From<Option<bool>> for OptBool {
	fn from(value: Option<bool>) -> Self {
		Self(value.unwrap_or(false))
	}
}
impl Deref for OptBool {
	type Target = bool;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Serialize)]
pub struct OptU32(pub u32);
impl From<u32> for OptU32 {
	fn from(value: u32) -> Self {
		Self(value)
	}
}
impl From<Option<u32>> for OptU32 {
	fn from(value: Option<u32>) -> Self {
		Self(value.unwrap_or(0))
	}
}
impl From<Option<i32>> for OptU32 {
	fn from(value: Option<i32>) -> Self {
		Self(value.unwrap_or(0) as u32)
	}
}
impl From<Option<i64>> for OptU32 {
	fn from(value: Option<i64>) -> Self {
		Self(value.unwrap_or(0) as u32)
	}
}
impl Deref for OptU32 {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}