use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, PlayerSide, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static _FORWARD_DRIBBLING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_dribbling_data.json")));

#[derive(Default)]
pub struct ForwardDribblingState {}

impl StateProcessingHandler for ForwardDribblingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if !ctx.player.has_ball(ctx) {
            // Transition to Running state if the player doesn't have the ball
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }
        
        if ctx.ball().distance_to_opponent_goal() < 250.0 && ctx.player().has_clear_shot() {
            return Some(StateChangeResult::with_forward_state(ForwardState::Shooting));
        }

        // Check if the player is under pressure
        if ctx.players().opponents().nearby_raw(50.0).count() >= 2 {
            // Transition to Passing state if under pressure
            return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
        }

        // Check if there's space to dribble forward
        if !self.has_space_to_dribble(ctx) {
            // Transition to HoldingUpPlay state if there's no space to dribble
            return Some(StateChangeResult::with_forward_state(
                ForwardState::HoldingUpPlay,
            ));
        }

        // Check if there's an opportunity to shoot
        if self.can_shoot(ctx) {
            // Transition to Shooting state if there's an opportunity to shoot
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Shooting,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.ball().direction_to_opponent_goal(),
                slowing_distance: 150.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl ForwardDribblingState {
    fn has_space_to_dribble(&self, ctx: &StateProcessingContext) -> bool {
        let dribble_distance = 10.0;
        let players = ctx.players();

        !players.opponents().exists(dribble_distance)
    }

    fn can_shoot(&self, ctx: &StateProcessingContext) -> bool {
        let shot_distance = 25.0; // Adjust based on your game's scale

        let distance_to_goal = ctx.ball().distance_to_opponent_goal();

        // Check if the player is within shooting distance and has a clear shot
        distance_to_goal < shot_distance && self.has_clear_shot(ctx)
    }

    fn has_clear_shot(&self, ctx: &StateProcessingContext) -> bool {
        let opponent_goal_position = match ctx.player.side {
            // swap for opponents
            Some(PlayerSide::Left) => ctx.context.goal_positions.left,
            Some(PlayerSide::Right) => ctx.context.goal_positions.right,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        ctx.players().opponents().all().all(|opponent| {
            let opponent_to_goal = (opponent_goal_position - opponent.position).normalize();
            let player_to_goal = (opponent_goal_position - ctx.player.position).normalize();
            opponent_to_goal.dot(&player_to_goal) < 0.9
        })
    }
}
