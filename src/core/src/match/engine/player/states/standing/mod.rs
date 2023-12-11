use crate::common::NeuralNetwork;

use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

use crate::r#match::position::VectorExtensions;
use nalgebra::Vector3;

lazy_static! {
    static ref PLAYER_STANDING_STATE_NETWORK: NeuralNetwork = PlayerStandingStateNetLoader::load();
}

pub struct StandingState {}

impl StandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if in_state_time > 20 {
            return Some(PlayerState::Walking);
        }

        if context.time.time % 1000 == 0 {}

        None
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_standing_data.json");

#[derive(Debug)]
pub struct PlayerStandingStateNetLoader;

impl PlayerStandingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}