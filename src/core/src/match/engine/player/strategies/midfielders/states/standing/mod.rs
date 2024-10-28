use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::{
    ConditionContext, PlayerSide, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use rand::Rng;
use std::sync::LazyLock;

static MIDFIELDER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

const PASSING_DISTANCE_THRESHOLD: f32 = 30.0; // Adjust as needed
const PRESSING_DISTANCE_THRESHOLD: f32 = 50.0; // Adjust as needed
const STAMINA_THRESHOLD: u32 = 20; // Minimum stamina percentage before resting

#[derive(Default)]
pub struct MidfielderStandingState {}

impl StateProcessingHandler for MidfielderStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.player_attributes.condition_percentage() < STAMINA_THRESHOLD {
            // Transition to Resting state
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Resting,
            ));
        }

        if ctx.in_state_time > 100 {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Running,
            ));
        }

        if !ctx.team().is_control_ball() && ctx.ball().distance() < 10.0 {
            // Transition to Tackling state to attempt to regain possession
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Tackling,
            ));
        }

        // 1. Check if the midfielder has the ball
        if ctx.player.has_ball {
            // Decide whether to hold possession or distribute the ball
            return if self.should_hold_possession(ctx) {
                Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::HoldingPossession,
                ))
            } else {
                Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Distributing,
                ))
            };
        }

        // 2. Check if the ball is close and the midfielder should attempt to gain possession
        if !ctx.team().is_control_ball() && ctx.ball().distance() < PRESSING_DISTANCE_THRESHOLD {
            // Transition to Tackling state to try and win the ball
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Pressing,
            ));
        }

        // 3. Check if an opponent is nearby and pressing is needed
        if self.is_opponent_nearby(ctx) {
            // Transition to Pressing state to apply pressure
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Pressing,
            ));
        }

        // 4. Check if a teammate is making a run and needs support
        if self.should_support_attack(ctx) {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::AttackSupporting,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic if necessary
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions
    }
}

impl MidfielderStandingState {
    /// Checks if the midfielder should hold possession based on game context.
    fn should_hold_possession(&self, ctx: &StateProcessingContext) -> bool {
        // For simplicity, let's assume the midfielder holds possession if there are no immediate passing options
        !self.has_passing_options(ctx)
    }

    fn calculate_movement_probability(
        &self,
        rng: &mut impl Rng,
        ctx: &StateProcessingContext,
    ) -> bool {
        let positioning_probability = (ctx.player.skills.mental.positioning as f64) / 20.0;
        let concentration_probability = (ctx.player.skills.mental.concentration as f64) / 20.0;

        let positioning_roll = rng.gen_range(0.0..1.0);
        let concentration_roll = rng.gen_range(0.0..1.0);

        positioning_roll < positioning_probability && concentration_roll < concentration_probability
    }

    /// Determines if the midfielder has passing options.
    fn has_passing_options(&self, ctx: &StateProcessingContext) -> bool {
        const PASSING_DISTANCE_THRESHOLD: f32 = 30.0; // Distance within which a teammate is considered available for a pass

        ctx.context
            .players
            .raw_players()
            .iter()
            .filter(|p| p.team_id == ctx.player.team_id && p.id != ctx.player.id)
            .any(|teammate| {
                let distance = (ctx.player.position - teammate.position).magnitude();
                distance < PASSING_DISTANCE_THRESHOLD
            })
    }

    const PRESSING_DISTANCE_THRESHOLD: f32 = 10.0;

    /// Checks if an opponent player is nearby within the pressing threshold.
    fn is_opponent_nearby(&self, ctx: &StateProcessingContext) -> bool {
         ctx.players().opponents().exists(PRESSING_DISTANCE_THRESHOLD)
    }

    /// Determines if the midfielder should support an attacking play.
    fn should_support_attack(&self, ctx: &StateProcessingContext) -> bool {
        // For simplicity, assume the midfielder supports the attack if the ball is in the attacking third
        let field_length = ctx.context.field_size.width as f32;
        let attacking_third_start = if ctx.player.side == Some(PlayerSide::Left) {
            field_length * (2.0 / 3.0)
        } else {
            field_length / 3.0
        };

        let ball_position_x = ctx.tick_context.object_positions.ball_position.x;

        if ctx.player.side == Some(PlayerSide::Left) {
            ball_position_x > attacking_third_start
        } else {
            ball_position_x < attacking_third_start
        }
    }
}
