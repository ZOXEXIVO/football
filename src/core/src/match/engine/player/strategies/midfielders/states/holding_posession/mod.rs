use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static MIDFIELDER_HOLDING_POSSESSION_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_holding_possession_data.json"))
});

const MAX_SHOOTING_DISTANCE: f32 = 300.0; // Maximum distance to attempt a shot
const MIN_SHOOTING_DISTANCE: f32 = 20.0; // Minimum distance to attempt a shot (e.g., edge of penalty area)

#[derive(Default)]
pub struct MidfielderHoldingPossessionState {}

impl StateProcessingHandler for MidfielderHoldingPossessionState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the midfielder has the ball
        if !ctx.player.has_ball {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Returning,
            ));
        }

        if self.is_in_shooting_range(ctx) {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Shooting,
            ));
        }

        let players= ctx.players();
        let teammates = players.teammates();

        if let Some(open_teammate) = teammates
            .nearby(300.0)
            .filter(|teammate| self.is_teammate_open(ctx, teammate)).next() {
            // If there is an open teammate, transition to the passing state
            return Some(StateChangeResult::with_midfielder_state_and_event(
                MidfielderState::ShortPassing,
                Event::PlayerEvent(PlayerEvent::PassTo(
                    ctx.player.id,
                    open_teammate.position,
                    1.0, // Adjust the pass power as needed
                )),
            ));
        }

        // Check if the midfielder is being pressured by opponents
        if self.is_under_pressure(ctx) {
            // If under pressure, decide whether to dribble or pass based on the situation
            if self.has_space_to_dribble(ctx) {
                // If there is space to dribble, transition to the dribbling state
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Dribbling,
                ));
            } else {
                // If there is no space to dribble, look for a quick pass
                if let Some(nearby_teammate) = ctx.players().teammates().nearby(150.0).next() {
                    // If there is a nearby teammate, transition to the passing state
                    return Some(StateChangeResult::with_midfielder_state_and_event(
                        MidfielderState::ShortPassing,
                        Event::PlayerEvent(PlayerEvent::PassTo(
                            ctx.player.id,
                            nearby_teammate.position,
                            0.8, // Adjust the pass power for a quick pass
                        )),
                    ));
                }
            }
        }

        // Check if the midfielder has held possession for too long
        if ctx.in_state_time > 200 {
            // If holding possession for too long, decide the next action based on the situation
            if self.is_in_attacking_position(ctx) {
                // If in an attacking position, transition to the shooting state
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Shooting,
                ));
            } else {
                let players = ctx.players();
                let teammates = players.teammates();
                let forwards = teammates.forwards();

                let forward_position_threshold = ctx.context.field_size.width as f32 * 0.75; // Adjust this value based on your game's field dimensions

                let nearest_forward = forwards
                    .filter(|teammate| {
                        // Check if the teammate's position is beyond the forward position threshold
                        teammate.position.x >= forward_position_threshold
                    })
                    .max_by(|a, b| {
                        // Prioritize teammates closer to the opponent's goal
                        let dist_a = ctx.context.field_size.width as f32 - a.position.x;
                        let dist_b = ctx.context.field_size.width as f32 - b.position.x;
                        dist_a.partial_cmp(&dist_b).unwrap()
                    });

                if let Some(forward_teammate) = nearest_forward {
                    // If there is a forward teammate, transition to the passing state
                    return Some(StateChangeResult::with_midfielder_state_and_event(
                        MidfielderState::ShortPassing,
                        Event::PlayerEvent(PlayerEvent::PassTo(
                            ctx.player.id,
                            forward_teammate.position,
                            1.2, // Adjust the pass power for a forward pass
                        )),
                    ));
                } else {
                    // If no forward teammate is available, transition to the dribbling state
                    return Some(StateChangeResult::with_midfielder_state(
                        MidfielderState::Dribbling,
                    ));
                }
            }
        }

        // If none of the above conditions are met, continue holding possession
        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.ball().direction_to_opponent_goal(),
                slowing_distance: 30.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl MidfielderHoldingPossessionState {
    pub fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        ctx.players().opponents().nearby_raw(30.0).count() >= 1
    }

    fn is_teammate_open(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        // Check if a teammate is open to receive a pass
        let is_in_passing_range = (teammate.position - ctx.player.position).magnitude() <= 30.0;
        let has_clear_passing_lane = self.has_clear_passing_lane(ctx, teammate);

        is_in_passing_range && has_clear_passing_lane
    }

    fn has_space_to_dribble(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player has space to dribble the ball
        let dribble_distance = 10.0; // Adjust this value based on your game's scale

        let players = ctx.players();
        let opponents = players.opponents();

        let mut nearby_opponents = opponents.nearby_raw(dribble_distance);

        nearby_opponents.all(|(_, distance)| distance > dribble_distance)
    }

    fn is_in_attacking_position(&self, ctx: &StateProcessingContext) -> bool {
        // Define the attacking position threshold
        let attacking_position_threshold = ctx.context.field_size.width as f32 * 0.75; // Adjust this value based on your game's field dimensions

        // Check if the player's position is beyond the attacking position threshold
        ctx.player.position.x >= attacking_position_threshold
    }

    fn has_clear_passing_lane(&self, ctx: &StateProcessingContext, teammate: &MatchPlayer) -> bool {
        // Check if there is a clear passing lane to a teammate without any obstructing opponents
        let player_position = ctx.player.position;
        let teammate_position = teammate.position;
        let passing_direction = (teammate_position - player_position).normalize();

        let ray_cast_result = ctx.tick_context.space.cast_ray(
            player_position,
            passing_direction,
            (teammate_position - player_position).magnitude(),
            false,
        );

        ray_cast_result.is_none() // No collisions with opponents
    }

    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();
        distance_to_goal <= MAX_SHOOTING_DISTANCE && distance_to_goal >= MIN_SHOOTING_DISTANCE
    }
}
