mod account;
mod crypto;
mod qwiz;

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;



use std::str::FromStr;

use rocket::{http::Status, Request};
use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgConnectOptions}, ConnectOptions};
use async_once::AsyncOnce;



pub static BASE_URL: &str = "/api";
lazy_static! {

	pub static ref POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
		
		let connection_url = dotenv::var("DATABASE_URL").unwrap();
		PgPoolOptions::new()
			.connect_with(
				PgConnectOptions::from_str(&connection_url).unwrap()
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

	rocket::build()
		.register(BASE_URL, catchers![default_catcher])
		.mount(BASE_URL, routes)

}



#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> String {
	status.code.to_string()
}

#[get("/")]
fn root_info() -> &'static str {
r#"
/account
"#
}
