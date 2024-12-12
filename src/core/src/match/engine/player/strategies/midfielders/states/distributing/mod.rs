use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::{PassingEventModel, PlayerEvent};
use crate::r#match::{
    ConditionContext, MatchPlayerLite, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use rand::prelude::IteratorRandom;
use std::sync::LazyLock;

static MIDFIELDER_DISTRIBUTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_distributing_data.json")));

#[derive(Default)]
pub struct MidfielderDistributingState {}

impl StateProcessingHandler for MidfielderDistributingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Find the best passing option
        if let Some(teammate) = self.find_best_pass_option(ctx) {
            return Some(StateChangeResult::with_midfielder_state_and_event(
                MidfielderState::Returning,
                Event::PlayerEvent(PlayerEvent::PassTo(
                    PassingEventModel::build()
                        .with_player_id(ctx.player.id)
                        .with_target(ctx.tick_context.positions.players.position(teammate.id))
                        .with_force(ctx.player().pass_teammate_power(teammate.id))
                        .build(),
                )),
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl MidfielderDistributingState {
    fn find_best_pass_option<'a>(
        &self,
        ctx: &StateProcessingContext<'a>,
    ) -> Option<MatchPlayerLite> {
        let players = ctx.players();

        if let Some(player) = players
            .teammates()
            .nearby(300.0)
            .choose(&mut rand::thread_rng())
        {
            return Some(player);
        }

        None
    }
}
