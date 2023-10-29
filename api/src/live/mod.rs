use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

use crate::question::Question;

pub mod routes;



pub struct LiveQwizOptions {
	shuffle_questions: bool,
	shuffle_answers: bool,
}

pub struct LiveQwizParticipant {
	id: i32,
	display_name: String,
	profile_picture_uri: String,
}

pub struct StartingLiveQwiz ();
pub struct RunningLiveQwiz {
	question_number: usize,
	current_answers: Vec<String>,
	correct_answer: u8,
	accepted_answers: HashMap<i32, Vec<bool>>,
}
impl RunningLiveQwiz {
	fn new(question: Question, shuffle_answers: bool, participant_ids: Vec<i32>) -> Self {

		let mut answers = vec![question.answer1.to_owned(), question.answer2.to_owned()];
		if let Some(a3) = &question.answer3 {
			answers.push(a3.to_owned());
		}
		if let Some(a4) = &question.answer4 {
			answers.push(a4.to_owned());
		}

		let mut indices: Vec<u8> = (0..(answers.len() as u8)).collect();
		if shuffle_answers {
			indices.shuffle(&mut thread_rng());
		}
		let correct_answer = indices.iter().position(|&r| r as i16 == question.correct - 1).unwrap() as u8;
		let current_answers = indices.into_iter().map(|i| answers[i as usize].to_owned()).collect();
		let accepted_answers = participant_ids.into_iter().map(|id| (id, Vec::new())).collect();

		Self { question_number: 0, current_answers, correct_answer, accepted_answers }
	}

	fn next(&self, next_question: Option<Question>, shuffle_answers: bool, participant_ids: Vec<i32>) -> LiveQwizState {
		match next_question {
			Some(nq) => {
				let question_number = self.question_number + 1;
				let mut next_answers = vec![nq.answer1, nq.answer2];
				if let Some(a3) = nq.answer3 {
					next_answers.push(a3);
				}
				if let Some(a4) = nq.answer4 {
					next_answers.push(a4);
				}

				let mut indices: Vec<u8> = (0..(next_answers.len() as u8)).collect();
				if shuffle_answers {
					indices.shuffle(&mut thread_rng());
				}
				let correct_answer = indices.iter().position(|&r| r as i16 == nq.correct - 1).unwrap() as u8;
				let current_answers = indices.into_iter().map(|i| next_answers[i as usize].to_owned()).collect();

				let mut accepted_answers = HashMap::new();
				for (&id, answers) in &self.accepted_answers {
					let mut new_answers = answers.clone();
					while new_answers.len() < question_number {
						new_answers.push(false);
					}
					accepted_answers.insert(id, new_answers);
				}

				LiveQwizState::Running(
					Self {
						question_number,
						current_answers,
						correct_answer,
						accepted_answers,
					}
				)
			},
			None => LiveQwizState::Finishing(FinishingLiveQwiz::from_running(self)),
		}
	}
}

pub struct FinishingLiveQwiz {
	scores: HashMap<i32, usize>,
}
impl FinishingLiveQwiz {
	fn new(participants: Vec<i32>) -> Self {
		Self {
			scores: participants.iter().map(|&id| (id, 0)).collect()
		}
	}

	fn from_running(value: &RunningLiveQwiz) -> Self {
		let scores: HashMap<i32, usize> = value.accepted_answers
			.iter()
			.map(
				|(&k, v)| (k, v.iter().filter(|&c| *c).count())
			)
			.collect();
		Self { scores }
	}
}

pub enum LiveQwizState {
	Starting(StartingLiveQwiz),
	Running(RunningLiveQwiz),
	Finishing(FinishingLiveQwiz),
}
impl LiveQwizState {
	fn next(&mut self, question: Option<Question>, shuffle_answers: bool, participant_ids: Vec<i32>) {
		match self {
			LiveQwizState::Starting(_) => {
				*self = match question {
					Some(q) => LiveQwizState::Running(RunningLiveQwiz::new(q, shuffle_answers, participant_ids)),
					None => LiveQwizState::Finishing(FinishingLiveQwiz::new(participant_ids)),
				};
			},
			LiveQwizState::Running(s) => {
				*self = s.next(question, shuffle_answers, participant_ids);
			},
			LiveQwizState::Finishing(_) => (),
		}
	}
}

pub struct LiveQwiz {
	qwiz_id: i32,
	options: LiveQwizOptions,
	questions: Box<dyn Iterator<Item = Question>>,
	participants: Vec<LiveQwizParticipant>,
	state: LiveQwizState,
}
impl LiveQwiz {
	fn new(qwiz_id: i32, options: LiveQwizOptions, questions: Vec<Question>, participants: Vec<LiveQwizParticipant>) -> Self {
		Self {
			qwiz_id,
			options,
			questions: Box::new(questions.into_iter()),
			participants,
			state: LiveQwizState::Starting(StartingLiveQwiz()),
		}
	}

	fn progress(&mut self) {
		use LiveQwizState::*;

		let participant_ids = self.participants.iter().map(|p| p.id).collect();
		match (&self.state, self.questions.next()) {
			(Starting(_), Some(q)) => {
				self.state = Running(RunningLiveQwiz::new(
					q,
					self.options.shuffle_answers,
					participant_ids,
				));
			},
			(Starting(_), None) => {
				self.state = Finishing(FinishingLiveQwiz::new(participant_ids));
			},
			(Running(state), q) => {
				self.state = state.next(q, self.options.shuffle_answers, participant_ids);
			},
			(Finishing(_), _) => (),
		};

	}
}
