use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior, MATCH_HALF_TIME_MS};
use crate::r#match::defenders::states::DefenderState;

static DEFENDER_RETURNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_returning_data.json")));

#[derive(Default)]
pub struct DefenderReturningState {}

impl StateProcessingHandler for DefenderReturningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.has_ball {
            return Some(StateChangeResult::with_defender_state(DefenderState::Passing));
        }

        // Stay in returning state until very close to start position
        if ctx.player().distance_from_start_position() < 2.0 {
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        // Intercept if ball coming towards player and is closer than before
        if ctx.ball().is_towards_player_with_angle(0.8) {
            return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
        }

        // Transition to Pressing late in the game only if ball is close as well
        if ctx.player().is_team_loosing() &&
            ctx.context.time.time > (MATCH_HALF_TIME_MS - 180) &&
            ctx.ball().distance() < 30.0 {
            return Some(StateChangeResult::with_defender_state(DefenderState::Pressing));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(SteeringBehavior::Arrive {
            target: ctx.player.start_position,
            slowing_distance: 10.0,
        }.calculate(ctx.player).velocity)
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}
