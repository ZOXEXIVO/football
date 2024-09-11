use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler};

static DEFENDER_SLIDING_TACKLE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_sliding_tackle_data.json")));

#[derive(Default)]
pub struct DefenderSlidingTackleState {}

impl StateProcessingHandler for DefenderSlidingTackleState {
    fn try_fast(&self, context: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, context: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
