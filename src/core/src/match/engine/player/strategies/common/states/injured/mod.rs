use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateProcessingHandler};
use std::sync::LazyLock;

static COMMON_INJURED_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_common_injured_data.json")));

#[derive(Default)]
pub struct CommonInjuredState {}

impl StateProcessingHandler for CommonInjuredState {
    fn try_fast(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::none()
    }
}
