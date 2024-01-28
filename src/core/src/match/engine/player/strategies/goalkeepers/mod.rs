﻿mod states;

use crate::r#match::position::VectorExtensions;
use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, StateChangeResult, SteeringBehavior};
use nalgebra::Vector3;
use crate::common::NeuralNetwork;
use crate::FloatUtils;

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn calculate(
        context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult {
        let is_ball_heading_towards_goal =
            ball_heading_towards_goal(objects_positions.ball_position, player.start_position);

        let ball_distance = objects_positions
            .ball_position
            .distance_to(&player.position);

        // match player.state {
        //     PlayerState::Standing => {
        //         //GoalkeeperStandingState::process(in_state_time, self, context, result, objects_positions)
        //     }
        //     PlayerState::Walking => {
        //         WalkingState::process(in_state_time, self, context, result, objects_positions)
        //     }
        //     PlayerState::Running => {
        //         RunningState::process(in_state_time, self, context, result, objects_positions)
        //     }
        //     PlayerState::Tackling => {
        //         TacklingState::process(in_state_time, self, context, result, objects_positions)
        //     }
        //     PlayerState::Shooting => {
        //         ShootingState::process(in_state_time, self, context, result, objects_positions)
        //     }
        //     PlayerState::Passing => {
        //         PassingState::process(in_state_time, self, context, result, objects_positions)
        //     }
        //     PlayerState::Returning => {
        //         ReturningState::process(in_state_time, self, context, result, objects_positions)
        //     }
        // }

        return match (ball_distance, is_ball_heading_towards_goal) {
            (0.0..=3.0, _) => {
                return StateChangeResult::with_velocity(Vector3::new(0.0, 0.0, 0.0));
            }
            (0.0..=10.0, _) => {
                let clear_target = Vector3::new(0.0, if player.position.y > 0.0 { 100.0 } else { -100.0 }, 0.0);
                return StateChangeResult::with_velocity(SteeringBehavior::Arrive {
                    target: clear_target,
                    slowing_distance: 5.0,
                }
                    .calculate(player)
                    .velocity);
            }
            (10.0..=100.0, true) => {
                StateChangeResult::with_velocity(SteeringBehavior::Arrive {
                    target: objects_positions.ball_position,
                    slowing_distance: 10.0 + ball_distance * 0.1,
                }.calculate(player)
                    .velocity)
            }
            _ => {
                let wander_velocity = SteeringBehavior::Wander {
                    target: player.start_position,
                    radius: 20.0,
                    jitter: 100.0,
                    distance: 60.0,
                    angle: FloatUtils::random(5.0, 90.0),
                }
                    .calculate(player)
                    .velocity;

                //println!("wander = {}", wander_velocity);

                StateChangeResult::with_velocity(wander_velocity)
            }
        };
    }
}

fn ball_heading_towards_goal(ball_position: Vector3<f32>, goal_position: Vector3<f32>) -> bool {
    let ball_to_goal = goal_position - ball_position;

    let ball_forward = Vector3::new(1.0, 0.0, 0.0);

    let dot_product = ball_to_goal.normalize().dot(&ball_forward);

    dot_product > 0.8
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct GoalkeepersNetLoader;

impl GoalkeepersNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}