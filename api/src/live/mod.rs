pub mod routes;

use crate::account::Account;
use crate::live::routes::{GetLiveQwizData, LiveQwizOptions};
use crate::qwiz::Qwiz;
use crate::question::Question;

use rand::{Rng, seq::SliceRandom};
use serde::Serialize;
use tokio::sync::Mutex;

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

	pub async fn new(host_id: i32, qwiz_id: i32, options: &LiveQwizOptions) -> sqlx::Result<Self> {

		let qwiz = Qwiz::get_by_id(&qwiz_id).await?;
		let mut questions = Question::get_all_by_qwiz_id(&qwiz_id).await?;

		let live_qwizes = LIVE_QWIZES.lock().await;

		let mut rng = rand::thread_rng();
		
		let mut id = 0u16;
		while id == 0 || live_qwizes.iter().any(|q| q.id == id) {
			id = rng.gen();
		}

		if options.shuffle_questions {
			questions.shuffle(&mut rng);
		}

		Ok( Self { id, host_id, qwiz_id, qwiz, questions, participants: vec![], state: LiveQwizState::Starting } )

	}

	pub async fn start(id: &u16) -> Result<(), LiveQwizError> {

		use LiveQwizState::*;
		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == *id)
			.ok_or(QwizNotFound(*id))?;

		if live_qwiz.state == Starting {
			live_qwiz.state = Live(0);
		}

		Ok(())

	}

	pub async fn next_question(id: &u16) -> Result<(), LiveQwizError> {

		use LiveQwizState::*;
		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == *id)
			.ok_or(QwizNotFound(*id))?;

		if let Live(current_question) = live_qwiz.state {
			if current_question + 1 >= live_qwiz.questions.len() {
				live_qwiz.state = Finished
			} else {
				live_qwiz.state = Live(current_question + 1)
			}
		}

		Ok(())

	}

	pub async fn delete(id: &u16) {

		LIVE_QWIZES.lock().await.retain(|q| &q.id != id);

	}

	pub async fn add_participants(id: &u16, participants: &Vec<PutLiveQwizParticipant>) -> Result<(), LiveQwizError> {

		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == *id)
			.ok_or(QwizNotFound(*id))?;
		
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

	pub async fn remove_participants(id: &u16, participant_ids: &[i32]) -> Result<(), LiveQwizError> {

		use LiveQwizError::QwizNotFound;

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == *id)
			.ok_or(QwizNotFound(*id))?;

		live_qwiz.participants.retain(|p| !participant_ids.contains(&p.id));

		Ok(())

	}

	pub async fn get_data(id: &u16) -> Option<GetLiveQwizData> {

		let mut live_qwizes = LIVE_QWIZES.lock().await;

		let live_qwiz = live_qwizes
			.iter_mut()
			.find(|q| q.id == *id)?;

		Some(GetLiveQwizData::from_live_qwiz(live_qwiz).await)

	}

}
