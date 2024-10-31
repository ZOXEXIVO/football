use crate::r#match::result::VectorExtensions;
use crate::r#match::{MatchPlayer, MatchPlayerLite, PlayerDistanceFromStartPosition, PlayerSide, StateProcessingContext};
use nalgebra::Vector3;
use crate::PlayerSkills;

pub struct PlayerOperationsImpl<'p> {
    ctx: &'p StateProcessingContext<'p>,
}

impl<'p> PlayerOperationsImpl<'p> {
    pub fn new(ctx: &'p StateProcessingContext<'p>) -> Self {
        PlayerOperationsImpl { ctx }
    }
}

impl<'p> PlayerOperationsImpl<'p> {
    pub fn get(&self, player_id: u32) -> MatchPlayerLite {
        MatchPlayerLite{
            id: player_id,
            position: self.ctx.tick_context.positions.players.position(player_id)
        }
    }

    pub fn skills(&self, player_id: u32) -> &PlayerSkills {
        let player = self.ctx.context.players.by_id(player_id).unwrap();
        &player.skills
    }

    pub fn on_own_side(&self) -> bool {
        let field_half_width = self.ctx.context.field_size.width / 2;

        if let Some(side) = self.ctx.player.side {
            return side == PlayerSide::Left
                && self.ctx.player.position.x < field_half_width as f32;
        }

        false
    }

    pub fn opponent_goal_position(&self) -> Vector3<f32> {
        match self.ctx.player.side {
            Some(PlayerSide::Left) => self.ctx.context.goal_positions.right,
            Some(PlayerSide::Right) => self.ctx.context.goal_positions.left,
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn distance_from_start_position(&self) -> f32 {
        self.ctx
            .player
            .start_position
            .distance_to(&self.ctx.player.position)
    }

    pub fn position_to_distance(&self) -> PlayerDistanceFromStartPosition {
        MatchPlayerLogic::distance_to_start_position(&self.ctx.player)
    }

    pub fn is_tired(&self) -> bool {
        self.ctx.player.player_attributes.condition_percentage() > 50
    }

    pub fn calculate_pass_power(&self, teammate: &MatchPlayer) -> f64 {
        let distance = self
            .ctx
            .tick_context
            .distances
            .get(self.ctx.player.id, teammate.id)
            .unwrap();
        let pass_skill = self.ctx.player.skills.technical.passing;
        (distance / pass_skill as f32 * 10.0) as f64
    }

    pub fn distance_to_player(&self, player_id: u32) -> f32 {
        self.ctx.tick_context
            .distances
            .get(self.ctx.player.id, player_id)
            .unwrap()
    }
}

pub struct MatchPlayerLogic;

impl MatchPlayerLogic {
    pub fn distance_to_start_position(player: &MatchPlayer) -> PlayerDistanceFromStartPosition {
        let start_position_distance = player.position.distance_to(&player.start_position);

        if start_position_distance < 50.0 {
            PlayerDistanceFromStartPosition::Small
        } else if start_position_distance < 100.0 {
            PlayerDistanceFromStartPosition::Medium
        } else {
            PlayerDistanceFromStartPosition::Big
        }
    }
}
