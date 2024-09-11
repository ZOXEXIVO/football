use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{StateProcessingContext, StateProcessingHandler};
use crate::r#match::strategies::processor::StateChangeResult;

static COMMON_INJURED_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_common_injured_data.json")));

#[derive(Default)]
pub struct CommonInjuredState {}

impl StateProcessingHandler for CommonInjuredState {
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
