use crate::r#match::result::VectorExtensions;
use crate::r#match::{
    MatchPlayer, MatchPlayerLite, PlayerDistanceFromStartPosition, PlayerSide,
    StateProcessingContext,
};
use crate::PlayerSkills;
use nalgebra::Vector3;

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
        MatchPlayerLite {
            id: player_id,
            position: self.ctx.tick_context.positions.players.position(player_id),
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
        MatchPlayerLogic::distance_to_start_position(self.ctx.player)
    }

    pub fn is_tired(&self) -> bool {
        self.ctx.player.player_attributes.condition_percentage() > 50
    }

    pub fn pass_teammate_power(&self, teammate_id: u32) -> f64 {
        let distance = self
            .ctx
            .tick_context
            .distances
            .get(self.ctx.player.id, teammate_id);

        let pass_skill = self.ctx.player.skills.technical.passing;

        let raw_power = (distance / pass_skill * 10.0) as f64;

        // Normalize the pass force based on a desired range
        let min_power = 3.0;
        let max_power = 10.0;
        let normalized_power = (raw_power - min_power) / (max_power - min_power);

        // Clamp the normalized power between 0.0 and 1.0
        let clamped_power = normalized_power.clamp(0.0, 1.0);

        clamped_power
    }

    pub fn distance_to_player(&self, player_id: u32) -> f32 {
        self.ctx
            .tick_context
            .distances
            .get(self.ctx.player.id, player_id)
    }
}

pub struct MatchPlayerLogic;

impl MatchPlayerLogic {
    pub fn distance_to_start_position(player: &MatchPlayer) -> PlayerDistanceFromStartPosition {
        let start_position_distance = player.position.distance_to(&player.start_position);

        if start_position_distance < 100.0 {
            PlayerDistanceFromStartPosition::Small
        } else if start_position_distance < 250.0 {
            PlayerDistanceFromStartPosition::Medium
        } else {
            PlayerDistanceFromStartPosition::Big
        }
    }
}
