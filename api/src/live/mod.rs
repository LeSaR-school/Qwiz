pub mod routes;

use core::fmt;
use std::time::Duration;

use crate::account::Account;
use crate::live::routes::{GetLiveQwizData, LiveQwizOptions};
use crate::log_err;
use crate::qwiz::Qwiz;
use crate::question::Question;

use rand::{Rng, seq::SliceRandom};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio::time;

use self::routes::PutLiveQwizParticipant;



lazy_static! {
	static ref LIVE_QWIZES: Mutex<Vec<LiveQwiz>> = Mutex::new(vec![]);
}



pub enum LiveQwizError {
	Sqlx(sqlx::Error),
	QwizNotFound(u16),
}
impl From<sqlx::Error> for LiveQwizError {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}
impl fmt::Display for LiveQwizError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Sqlx(e) => e.fmt(f),
			Self::QwizNotFound(id) => write!(f, "Qwiz with id {id} not found"),
		}
	}
}

pub enum StartLiveQwizError {
	LiveQwiz(LiveQwizError),
	QwizAlreadyStarted,
}
impl From<LiveQwizError> for StartLiveQwizError {
	fn from(value: LiveQwizError) -> Self {
		Self::LiveQwiz(value)
	}
}
impl From<sqlx::Error> for StartLiveQwizError {
	fn from(value: sqlx::Error) -> Self {
		Self::from(LiveQwizError::from(value))
	}
}

#[derive(Clone, PartialEq, Serialize)]
pub enum LiveQwizState {
	Starting,
	Live(usize),
	Finished,
}

#[derive(Serialize, Clone)]
pub struct LiveQwizParticipant {
	pub id: i32,
	pub display_name: String,
	ready: bool,
}

pub struct LiveQwiz {
	id: u16,
	host_id: i32,
	participants: Vec<LiveQwizParticipant>,
	qwiz_id: i32,
	qwiz: Qwiz,
	questions: Vec<Question>,
	state: LiveQwizState,
}

impl LiveQwiz {

	pub async fn new(host_id: i32, qwiz_id: i32, options: &LiveQwizOptions) -> sqlx::Result<u16> {

		let qwiz = Qwiz::get_by_id(&qwiz_id).await?;
		let mut questions = Question::get_all_by_qwiz_id(&qwiz_id).await?;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let mut rng = rand::thread_rng();
		
		let mut id = 0u16;
		while id == 0 || live_qwizes.iter().any(|q| q.id == id) {
			id = rng.gen();
		}

		if options.shuffle_questions {
			questions.shuffle(&mut rng);
		}



		let live_qwiz = Self { id, host_id, qwiz_id, qwiz, questions, participants: vec![], state: LiveQwizState::Starting };
		live_qwizes.push(live_qwiz);
		Ok(id)

	}

	pub async fn start(id: u16) -> Result<(), StartLiveQwizError> {

		use LiveQwizState::*;
		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == id)
			.ok_or(QwizNotFound(id))?;

		if live_qwiz.state != Starting {
			return Err(StartLiveQwizError::QwizAlreadyStarted)
		}
		
		tokio::spawn(async move {
			let mut interval = time::interval(Duration::from_secs(10));
			loop {
				interval.tick().await;
				match LiveQwiz::next_question(id).await {
					Ok(false) => (),
					Ok(true) => break,
					Err(e) => {
						log_err(&e);
						break
					},
				}
			}

			interval.tick().await;
	
			LiveQwiz::delete(id).await;
		});

		Ok(())

	}

	pub async fn next_question(id: u16) -> Result<bool, LiveQwizError> {

		use LiveQwizState::*;
		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == id)
			.ok_or(QwizNotFound(id))?;

		match live_qwiz.state {
			Starting => {
				live_qwiz.state = Live(0);
				Ok(false)
			},
			Live(current_question) => {
				if current_question + 1 >= live_qwiz.questions.len() {
					live_qwiz.state = Finished;
					Ok(true)
				} else {
					live_qwiz.state = Live(current_question + 1);
					Ok(false)
				}
			},
			Finished => Ok(true),
		}

	}

	pub async fn delete(id: u16) {

		LIVE_QWIZES.lock().await.retain(|q| q.id != id);

	}

	pub async fn add_participants(id: u16, participants: &Vec<PutLiveQwizParticipant>) -> Result<(), LiveQwizError> {

		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == id)
			.ok_or(QwizNotFound(id))?;
		
		for participant in participants {

			if live_qwiz.participants.iter().any(|p| p.id == participant.id) {
				continue;
			}

			if let Ok(account) = Account::get_by_id(&participant.id).await {

				let display_name = match &participant.display_name {
					Some(name) => name.to_owned(),
					None => account.username,
				};

				live_qwiz.participants.push(
					LiveQwizParticipant { id: participant.id, display_name, ready: false }
				);
			
			}

		}

		Ok(())

	}

	pub async fn remove_participants(id: u16, participant_ids: &[i32]) -> Result<(), LiveQwizError> {

		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == id)
			.ok_or(QwizNotFound(id))?;

		live_qwiz.participants.retain(|p| !participant_ids.contains(&p.id));

		Ok(())

	}

	pub async fn get_data(id: u16) -> Option<GetLiveQwizData> {

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == id)?;

		Some(GetLiveQwizData::from_live_qwiz(live_qwiz).await)

	}

}
