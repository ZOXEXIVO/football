use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior, VectorExtensions,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_PRESAVE_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_preparing_for_save_data.json"))
});

#[derive(Default)]
pub struct GoalkeeperPreparingForSaveState {}

impl StateProcessingHandler for GoalkeeperPreparingForSaveState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.has_ball(ctx) {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Passing,
            ));
        } else {
            if ctx.team().is_control_ball() {
                return Some(StateChangeResult::with_goalkeeper_state(
                    GoalkeeperState::ReturningToGoal
                ));
            }
            
            // Transition to Walking if the ball is far away
            if ctx.ball().distance() > 250.0 {
                return Some(StateChangeResult::with_goalkeeper_state(
                    GoalkeeperState::Walking,
                ));
            }

            // Transition to Diving if the ball is close and moving fast towards goal
            if self.should_dive(ctx) {
                return Some(StateChangeResult::with_goalkeeper_state(
                    GoalkeeperState::Diving,
                ));
            }

            // Transition to Catching if the ball is catchable
            if self.is_ball_catchable(ctx) {
                return Some(StateChangeResult::with_goalkeeper_state(
                    GoalkeeperState::Catching,
                ));
            }

            // Transition to Coming Out if necessary
            if self.should_come_out(ctx) {
                return Some(StateChangeResult::with_goalkeeper_state(
                    GoalkeeperState::ComingOut,
                ));
            }
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Pursuit {
                target: ctx.tick_context.positions.ball.position,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl GoalkeeperPreparingForSaveState {
    fn should_dive(&self, ctx: &StateProcessingContext) -> bool {
        let ball_velocity = ctx.tick_context.positions.ball.velocity;
        let ball_distance = ctx.ball().distance();

        if ball_distance > 10.0 {
            return false;
        }
        
        // Check if the ball is moving fast towards the goal
        ball_velocity.dot(&(ctx.ball().direction_to_own_goal() - ctx.player.position)) > 0.0
    }

    fn is_ball_catchable(&self, ctx: &StateProcessingContext) -> bool {
        let ball_distance = ctx.ball().distance();
        let ball_speed = ctx.tick_context.positions.ball.velocity.norm();
        let goalkeeper_reach = ctx.player.skills.physical.jumping * 0.5 + 2.0; // Adjust as needed

        ball_distance < goalkeeper_reach && ball_speed < 10.0
    }

    fn should_come_out(&self, ctx: &StateProcessingContext) -> bool {
        let ball_distance = ctx.ball().distance();
        let goalkeeper_skills = &ctx.player.skills;

        ball_distance < 150.0 && goalkeeper_skills.mental.decisions > 8.0
    }
    
    fn calculate_optimal_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let goal_position = ctx.ball().direction_to_own_goal();
        let ball_position = ctx.tick_context.positions.ball.position;

        // Calculate a position on the line between the ball and the center of the goal
        let to_ball = ball_position - goal_position;
        let goal_line_width = 7.32; // Standard goal width in meters
        let optimal_distance = (goal_line_width / 2.0) * 0.9; // Position slightly inside the goal

        goal_position + to_ball.normalize() * optimal_distance
    }
}
