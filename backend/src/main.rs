mod account;
mod crypto;
mod qwiz;

extern crate sqlx;
extern crate tokio;

use std::str::FromStr;
use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgConnectOptions}, ConnectOptions};



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

	let POOL: Pool<Postgres> = PgPoolOptions::new()
	.connect_with(
		PgConnectOptions::from_str(dotenv::var("DATABASE_URL").unwrap().as_str())
			.unwrap()
			.disable_statement_logging()
			.clone()
	).await.unwrap();

	let thumbnail_url = "";



	Ok(())

}
