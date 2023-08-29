mod account;
mod qwiz;
mod question;
mod vote;
mod class;
mod media;
mod crypto;



#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;



use std::str::FromStr;
use rocket::{http::Status, Request, fs::{FileServer, relative}};
use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgConnectOptions}, ConnectOptions};
use async_once::AsyncOnce;



pub static BASE_URL: &str = "/api";
lazy_static! {

	pub static ref POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
		
		PgPoolOptions::new()
			.connect_with(
				PgConnectOptions::from_str(env!("DATABASE_URL")).unwrap()
				.disable_statement_logging().clone()
			)
			.await
			.unwrap()

	});

}



#[launch]
fn rocket() -> _ {

	let mut routes = routes![root_info];
	routes.append(&mut account::routes::all());
	routes.append(&mut qwiz::routes::all());
	routes.append(&mut question::routes::all());
	routes.append(&mut vote::routes::all());
	routes.append(&mut class::routes::all());
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
/vote
/media
"#
}
