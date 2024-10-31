use crate::r#match::result::VectorExtensions;
use crate::r#match::{BallSide, PlayerSide, StateProcessingContext};
use nalgebra::Vector3;

pub struct BallOperationsImpl<'b> {
    ctx: &'b StateProcessingContext<'b>,
}

impl<'b> BallOperationsImpl<'b> {
    pub fn new(ctx: &'b StateProcessingContext<'b>) -> Self {
        BallOperationsImpl { ctx }
    }
}

impl<'b> BallOperationsImpl<'b> {
    pub fn on_own_side(&self) -> bool {
        match self.side() {
            BallSide::Left => self.ctx.player.side == Some(PlayerSide::Left),
            BallSide::Right => self.ctx.player.side == Some(PlayerSide::Right),
        }
    }

    pub fn distance(&self) -> f32 {
        self.ctx
            .tick_context
            .positions
            .ball.position
            .distance_to(&self.ctx.player.position)
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        self.ctx.tick_context.positions.ball.velocity.norm()
    }

    #[inline]
    pub fn is_owned(&self) -> bool {
        self.ctx.tick_context.ball.is_owned
    }

    #[inline]
    pub fn owner_id(&self) -> Option<u32> {
        self.ctx.tick_context.ball.current_owner
    }

    pub fn is_towards_player(&self) -> bool {
        let (is_towards, _) = MatchBallLogic::is_heading_towards_player(
            &self.ctx.tick_context.positions.ball.position,
            &self.ctx.tick_context.positions.ball.velocity,
            &self.ctx.player.position,
            0.95,
        );
        is_towards
    }

    pub fn is_towards_player_with_angle(&self, angle: f32) -> bool {
        let (is_towards, _) = MatchBallLogic::is_heading_towards_player(
            &self.ctx.tick_context.positions.ball.position,
            &self.ctx.tick_context.positions.ball.velocity,
            &self.ctx.player.position,
            angle,
        );
        is_towards
    }

    pub fn distance_to_own_goal(&self) -> f32 {
        let target_goal = match self.ctx.player.side {
            Some(PlayerSide::Left) => Vector3::new(
                self.ctx.context.goal_positions.left.x,
                self.ctx.context.goal_positions.left.y,
                0.0,
            ),
            Some(PlayerSide::Right) => Vector3::new(
                self.ctx.context.goal_positions.right.x,
                self.ctx.context.goal_positions.right.y,
                0.0,
            ),
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        self.ctx
            .tick_context
            .positions
            .ball.position
            .distance_to(&target_goal)
    }

    pub fn direction_to_own_goal(&self) -> Vector3<f32> {
        match self.ctx.player.side {
            Some(PlayerSide::Left) => self.ctx.context.goal_positions.left,
            Some(PlayerSide::Right) => self.ctx.context.goal_positions.right,
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn direction_to_opponent_goal(&self) -> Vector3<f32> {
        let player_position = self.ctx.player.position;
        let ball_position = self.ctx.tick_context.positions.ball.position;
        let opponent_goal_position = self.ctx.player().opponent_goal_position();

        return opponent_goal_position;

        let players = self.ctx.players();
        let opponents = players.opponents();
        let mut goalkeepers = opponents.goalkeeper();
        let goalkeeper = goalkeepers.next().unwrap();
        let goalkeeper_position = goalkeeper.position;

        // Calculate the direction from the ball to the opponent's goal
        let ball_to_goal = opponent_goal_position - ball_position;

        // Calculate the direction from the ball to the goalkeeper
        let ball_to_goalkeeper = goalkeeper_position - ball_position;

        // Calculate the perpendicular direction to the goalkeeper
        let perpendicular_direction = Vector3::new(-ball_to_goalkeeper.y, ball_to_goalkeeper.x, 0.0);

        // Normalize the perpendicular direction
        let perpendicular_direction = perpendicular_direction.normalize();

        // Calculate the target position by offsetting from the goal center
        let goal_width = self.ctx.context.field_size.width as f32;
        let offset_distance = goal_width * 0.2; // Adjust the offset distance as needed
        let target_position = if player_position.x < ball_position.x {
            opponent_goal_position + perpendicular_direction * offset_distance
        } else {
            opponent_goal_position - perpendicular_direction * offset_distance
        };

        // Calculate the direction from the ball to the target position
        let ball_to_target = target_position - ball_position;

        // Normalize the direction vector
        let direction = ball_to_target.normalize();

        // Add some randomness to the direction based on the player's finishing skill
        let finishing_skill = self.ctx.player.skills.technical.finishing;
        let random_angle = (rand::random::<f32>() - 0.5) * finishing_skill.to_radians();
        let rotation_matrix = nalgebra::Rotation2::new(random_angle);
        let randomized_direction = rotation_matrix * direction.xy();

        Vector3::new(randomized_direction.x, randomized_direction.y, 0.0)
    }

    pub fn distance_to_opponent_goal(&self) -> f32 {
        let target_goal = match self.ctx.player.side {
            Some(PlayerSide::Left) => Vector3::new(
                self.ctx.context.goal_positions.right.x,
                self.ctx.context.goal_positions.right.y,
                0.0,
            ),
            Some(PlayerSide::Right) => Vector3::new(
                self.ctx.context.goal_positions.left.x,
                self.ctx.context.goal_positions.left.y,
                0.0,
            ),
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        self.ctx
            .tick_context
            .positions
            .ball.position
            .distance_to(&target_goal)
    }

    pub fn on_own_third(&self) -> bool {
        let field_length = self.ctx.context.field_size.width as f32;
        let ball_x = self.ctx.tick_context.positions.ball.position.x;

        if self.ctx.player.side == Some(PlayerSide::Left) {
            // Home team defends the left side (negative X)
            ball_x < -field_length / 3.0
        } else {
            // Away team defends the right side (positive X)
            ball_x > field_length / 3.0
        }
    }

    #[inline]
    pub fn side(&self) -> BallSide {
        if (self.ctx.tick_context.positions.ball.position.x as usize) <= self.ctx.context.field_size.half_width {
            return BallSide::Left;
        }

        BallSide::Right
    }
}

pub struct MatchBallLogic;

impl MatchBallLogic {
    pub fn is_heading_towards_player(
        ball_position: &Vector3<f32>,
        ball_velocity: &Vector3<f32>,
        player_position: &Vector3<f32>,
        angle: f32,
    ) -> (bool, f32) {
        Self::is_heading_towards_player_witj_angle(
            ball_position,
            ball_velocity,
            player_position,
            angle,
        )
    }

    pub fn is_heading_towards_player_witj_angle(
        ball_position: &Vector3<f32>,
        ball_velocity: &Vector3<f32>,
        player_position: &Vector3<f32>,
        angle: f32,
    ) -> (bool, f32) {
        let velocity_xy = Vector3::new(ball_velocity.x, ball_velocity.y, 0.0);
        let ball_to_player_xy = Vector3::new(
            player_position.x - ball_position.x,
            player_position.y - ball_position.y,
            0.0,
        );

        let velocity_norm = velocity_xy.norm();
        let direction_norm = ball_to_player_xy.norm();

        let normalized_velocity = velocity_xy / velocity_norm;
        let normalized_direction = ball_to_player_xy / direction_norm;
        let dot_product = normalized_velocity.dot(&normalized_direction);

        (dot_product >= angle, dot_product)
    }
}
