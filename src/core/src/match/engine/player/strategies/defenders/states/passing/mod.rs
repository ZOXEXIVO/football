use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::events::{Event, EventCollection};
use crate::r#match::player::events::{PassingEventModel, PlayerEvent};
use crate::r#match::{ConditionContext, MatchPlayerLite, PlayerSide, StateChangeResult, StateProcessingContext, StateProcessingHandler, VectorExtensions};
use nalgebra::Vector3;
use rand::prelude::IteratorRandom;
use std::sync::LazyLock;

static DEFENDER_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_passing_data.json")));

#[derive(Default)]
pub struct DefenderPassingState {}

impl StateProcessingHandler for DefenderPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if !ctx.player.has_ball(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Standing,
            ));
        }

        if let Some(teammate_id) = self.find_best_pass_option(ctx) {
            let teammate_player_position = ctx.tick_context.positions.players.position(teammate_id);

            return Some(StateChangeResult::with_defender_state_and_event(
                DefenderState::Returning,
                Event::PlayerEvent(PlayerEvent::PassTo(
                    PassingEventModel::build()
                        .with_player_id(ctx.player.id)
                        .with_target(teammate_player_position)
                        .with_force(ctx.player().pass_teammate_power(teammate_id))
                        .build()
                )),
            ));
        }
        let mut best_player_id = None;
        let mut highest_score = 0.0;

        for (player_id, teammate_distance) in ctx.players().teammates().nearby_ids(30.0) {
            let score = 1.0 / (teammate_distance + 1.0);
            if score > highest_score {
                highest_score = score;
                best_player_id = Some(player_id);
            }
        }

        if let Some(teammate_id) = best_player_id {
            let events = EventCollection::with_event(Event::PlayerEvent(PlayerEvent::PassTo(
                PassingEventModel::build()
                    .with_player_id(ctx.player.id)
                    .with_target(ctx.tick_context.positions.players.position(teammate_id))
                    .with_force(ctx.player().pass_teammate_power(teammate_id))
                    .build(),
            )));

            return Some(StateChangeResult::with_events(events));
        }

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

impl DefenderPassingState {
    fn find_best_pass_option<'a>(&'a self, ctx: &'a StateProcessingContext<'a>) -> Option<u32> {
        let teammates = ctx.players().teammates();
        let nearby_teammates = teammates.nearby(300.0);

        let mut nearest_to_goal: Option<MatchPlayerLite> = None;
        let mut min_to_goal_distance = f32::MAX;

        for teammate in nearby_teammates {
            let player = ctx
                .context
                .players
                .by_id(teammate.id)
                .expect(&format!("can't find player with id = {}", teammate.id));

            if player.tactical_position.current_position.is_goalkeeper() {
                continue;
            }

            let opponent_goal_position = match ctx.player.side {
                Some(PlayerSide::Left) => ctx.context.goal_positions.right,
                Some(PlayerSide::Right) => ctx.context.goal_positions.left,
                _ => Vector3::new(0.0, 0.0, 0.0),
            }.distance_to(&player.position);

            if opponent_goal_position < min_to_goal_distance {
                min_to_goal_distance = opponent_goal_position;
                nearest_to_goal = Some(teammate);
            }
        }

        if let Some(nearest) = nearest_to_goal {
            return Some(nearest.id);
        }

        None
    }

    pub fn calculate_pass_power(&self, teammate_id: u32, ctx: &StateProcessingContext) -> f64 {
        let distance = ctx.tick_context.distances.get(ctx.player.id, teammate_id);

        let pass_skill = ctx.player.skills.technical.passing;

        (distance / pass_skill as f32 * 10.0) as f64
    }
}
