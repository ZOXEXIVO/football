use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::EventCollection;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_SHOOTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_shooting_data.json")));

const SHOOTING_DISTANCE_THRESHOLD: f32 = 30.0; // Maximum distance to consider shooting
const SHOT_POWER_MULTIPLIER: f32 = 1.5; // Multiplier for shot power calculation

#[derive(Default)]
pub struct GoalkeeperShootingState {}

impl StateProcessingHandler for GoalkeeperShootingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the goalkeeper has the ball
        if !ctx.player.has_ball {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Standing,
            ));
        }

        // 3. Calculate the shooting direction and power
        let shooting_direction = ctx.ball().direction_to_opponent_goal();

        // 4. Shoot the ball towards the opponent's goal
        let mut events = EventCollection::new();

        events.add_player_event(PlayerEvent::Shoot(ctx.player.id, shooting_direction));

        Some(StateChangeResult::with_events(events))
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Remain stationary while shooting
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}
