use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static MIDFIELDER_DISTANCE_SHOOTING_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_distance_shooting_data.json"))
});

#[derive(Default)]
pub struct MidfielderDistanceShootingState {}

impl StateProcessingHandler for MidfielderDistanceShootingState {
    fn try_fast(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
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
