use rand::prelude::*;
use rand::thread_rng;

use teloxide::types::InputFile;

pub struct Action {
    pub condition: fn(&str) -> bool,
    pub consequence: ActionType,
}

#[derive(Clone)]
pub enum ActionType {
    Text(&'static str),
    Photo(InputFile),
}

pub fn random_probability(probability: f64) -> bool {
    thread_rng().gen_range(0.0..1.0) < probability
}
