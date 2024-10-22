use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_PRESSURE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_pressure_data.json")));

const PRESSURE_DISTANCE_THRESHOLD: f32 = 20.0; // Maximum distance from the goal to be considered under pressure
const COLLISION_DISTANCE_THRESHOLD: f32 = 2.0; // Distance threshold for considering a collision with the ball

#[derive(Default)]
pub struct GoalkeeperPressureState {}

impl StateProcessingHandler for GoalkeeperPressureState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the goalkeeper is within the pressure distance threshold from the goal
        let distance_to_goal = ctx.player.position.distance_to(&ctx.player.start_position);
        if distance_to_goal > PRESSURE_DISTANCE_THRESHOLD {
            // Goalkeeper is not under pressure, transition to appropriate state (e.g., Standing)
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Standing,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Move towards the start position (goal) using steering behavior
        let to_start_position = SteeringBehavior::Seek {
            target: ctx.player.start_position,
        }
        .calculate(ctx.player)
        .velocity;

        Some(to_start_position)
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}
