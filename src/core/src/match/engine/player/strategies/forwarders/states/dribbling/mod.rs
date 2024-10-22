use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    ConditionContext, MatchPlayer, PlayerSide, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static FORWARD_DRIBBLING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_dribbling_data.json")));

#[derive(Default)]
pub struct ForwardDribblingState {}

impl StateProcessingHandler for ForwardDribblingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();
        let player_ops = ctx.player();

        // Check if the player has the ball
        if !ctx.player.has_ball {
            // Transition to Running state if the player doesn't have the ball
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Check if the player is under pressure
        if player_ops.is_under_pressure() {
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

        // Dribble towards the opponent's goal
        let direction = ctx.ball().direction_to_opponent_goal();

        result.velocity = Some(direction * ctx.player.skills.physical.acceleration);

        Some(result)
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
        let dribble_distance = 10.0; // Adjust based on your game's scale
        let players = ctx.players();

        !players.opponents().exists_with_distance(dribble_distance)
    }

    fn is_open_for_pass(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        let max_distance = 20.0; // Adjust based on your game's scale

        // Check if the teammate is within a reasonable distance
        if let Some(distance) = ctx
            .tick_context
            .object_positions
            .player_distances
            .get(ctx.player.id, teammate.id)
        {
            if distance > max_distance {
                return false;
            }
        }

        let players = ctx.players();

        !players.opponents().exists_with_distance(5.0)
    }

    fn in_passing_lane(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let player_to_ball = (ball_position - ctx.player.position).normalize();
        let player_to_teammate = (teammate.position - ctx.player.position).normalize();

        // Check if the teammate is in the passing lane
        player_to_ball.dot(&player_to_teammate) > 0.8
    }

    fn scoring_chance(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> f32 {
        let goal_position = match teammate.side {
            Some(PlayerSide::Left) => ctx.context.goal_positions.right,
            Some(PlayerSide::Right) => ctx.context.goal_positions.left,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        let distance_to_goal = teammate.position.distance_to(&goal_position);

        // Calculate the scoring chance based on distance to the goal
        1.0 - distance_to_goal / ctx.context.field_size.width as f32
    }

    fn can_shoot(&self, ctx: &StateProcessingContext) -> bool {
        let shot_distance = 25.0; // Adjust based on your game's scale

        let distance_to_goal = ctx.ball().distance_to_opponent_goal();

        // Check if the player is within shooting distance and has a clear shot
        distance_to_goal < shot_distance && self.has_clear_shot(ctx)
    }

    fn has_clear_shot(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.players();
        let opponents = players.opponents();

        let opponent_goal_position = match ctx.player.side {
            // swap for opponents
            Some(PlayerSide::Left) => ctx.context.goal_positions.left,
            Some(PlayerSide::Right) => ctx.context.goal_positions.right,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        // Check if there are no opponents blocking the shot
        opponents.all().iter().all(|opponent| {
            let opponent_to_goal = (opponent_goal_position - opponent.position).normalize();
            let player_to_goal = (opponent_goal_position - ctx.player.position).normalize();
            opponent_to_goal.dot(&player_to_goal) < 0.9
        })
    }
}
