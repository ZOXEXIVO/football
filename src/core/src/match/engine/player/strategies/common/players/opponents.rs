use crate::r#match::{MatchPlayerLite, StateProcessingContext};
use crate::PlayerFieldPositionGroup;

pub struct PlayerOpponentsOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl<'b> PlayerOpponentsOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        PlayerOpponentsOperationsImpl { ctx }
    }
}

impl<'b> PlayerOpponentsOperationsImpl<'b> {
    pub fn all(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.opponents_for_team(self.ctx.player.team_id, None)
    }

    pub fn with_ball(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.opponents_for_team(self.ctx.player.team_id, Some(true))
    }

    pub fn without_ball(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.opponents_for_team(self.ctx.player.team_id, Some(false))
    }

    pub fn nearby(&self, distance: f32) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.ctx
            .tick_context
            .distances
            .opponents(self.ctx.player, distance)
            .map(|(pid, _)| MatchPlayerLite {
                id: pid,
                position: self.ctx.tick_context.positions.players.position(pid),
                tactical_positions: self.ctx.context.players.by_id(pid).expect(&format!(
                    "unknown player = {}", pid
                )).tactical_position.current_position
            })
    }

    pub fn nearby_raw(&self, distance: f32) -> impl Iterator<Item = (u32, f32)> + 'b {
        self.ctx
            .tick_context
            .distances
            .opponents(self.ctx.player, distance)
    }

    pub fn exists(&self, distance: f32) -> bool {
        self.ctx
            .tick_context
            .distances
            .opponents(self.ctx.player, distance)
            .any(|_| true)
    }

    pub fn goalkeeper(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.opponents_by_position(
            PlayerFieldPositionGroup::Goalkeeper,
            self.ctx.player.team_id,
        )
    }

    pub fn forwards(&'b self) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.opponents_by_position(PlayerFieldPositionGroup::Forward, self.ctx.player.team_id)
    }

    fn opponents_by_position(
        &'b self,
        position_group: PlayerFieldPositionGroup,
        team_id: u32,
    ) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                player.team_id != team_id
                    && player.tactical_position.current_position.position_group() == position_group
            })
            .map(|player| MatchPlayerLite {
                id: player.id,
                position: self.ctx.tick_context.positions.players.position(player.id),
                tactical_positions: player.tactical_position.current_position
            })
    }

    fn opponents_for_team(
        &'b self,
        team_id: u32,
        has_ball: Option<bool>,
    ) -> impl Iterator<Item = MatchPlayerLite> + 'b {
        self.ctx
            .context
            .players
            .players
            .values()
            .filter(move |player| {
                // opponent
                player.team_id != team_id
                    && (has_ball.is_none()
                        || (self.ctx.ball().owner_id() == Some(self.ctx.player.id)))
            })
            .map(|player| MatchPlayerLite {
                id: player.id,
                position: self.ctx.tick_context.positions.players.position(player.id),
                tactical_positions: player.tactical_position.current_position
            })
    }
}
