use crate::r#match::{MatchField, MatchPlayer, VectorExtensions};
use log::debug;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct PlayerDistanceClosure {
    pub distances: BinaryHeap<PlayerDistanceItem>,
}

pub struct PlayerDistanceItem {
    pub player_from_id: u32,
    pub player_from_team: u32,
    //pub player_from_position: Vector3<f32>,
    pub player_to_id: u32,
    pub player_to_team: u32,
    //pub player_to_position: Vector3<f32>,
    pub distance: f32,
}

impl From<&MatchField> for PlayerDistanceClosure {
    fn from(field: &MatchField) -> Self {
        let mut distances = BinaryHeap::with_capacity(50);

        for outer_player in &field.players {
            for inner_player in &field.players {
                if outer_player.id == inner_player.id {
                    continue;
                }

                let distance = outer_player.position.distance_to(&inner_player.position);

                distances.push(PlayerDistanceItem {
                    player_from_id: outer_player.id,
                    player_from_team: outer_player.team_id,
                    player_to_id: inner_player.id,
                    player_to_team: inner_player.team_id,
                    distance,
                });

                distances.push(PlayerDistanceItem {
                    player_from_id: inner_player.id,
                    player_from_team: inner_player.team_id,
                    player_to_id: outer_player.id,
                    player_to_team: outer_player.team_id,
                    distance,
                });
            }
        }

        PlayerDistanceClosure { distances }
    }
}

impl PlayerDistanceClosure {
    pub fn get(&self, player_from_id: u32, player_to_id: u32) -> f32 {
        if player_from_id == player_to_id {
            debug!(
                "player {} and {} are the same",
                player_from_id, player_to_id
            );
            return 0.0;
        }
        self.distances
            .iter()
            .find(|distance| {
                (distance.player_from_id == player_from_id && distance.player_to_id == player_to_id)
                    || (distance.player_from_id == player_to_id
                        && distance.player_to_id == player_from_id)
            })
            .map(|dist| dist.distance)
            .expect(&format!(
                "no distance between {} and {}",
                player_from_id, player_to_id
            ))
    }

    pub fn get_collisions(&self, max_distance: f32) -> Vec<&PlayerDistanceItem> {
        self.distances
            .iter()
            .filter(|&p| p.distance < max_distance)
            .collect()
    }

    pub fn teammates<'t>(
        &'t self,
        player: &'t MatchPlayer,
        distance: f32,
    ) -> impl Iterator<Item = (u32, f32)> + 't {
        self.distances
            .iter()
            .filter(move |p| p.distance <= distance)
            .filter(|item| {
                item.player_from_id == player.id
                    && item.player_from_team == item.player_to_team
                    && item.player_from_id != item.player_to_id
            })
            .map(|item| (item.player_to_id, item.distance))
    }

    pub fn opponents<'t>(
        &'t self,
        player: &'t MatchPlayer,
        distance: f32,
    ) -> impl Iterator<Item = (u32, f32)> + 't {
        self.distances
            .iter()
            .filter(move |p| p.distance <= distance)
            .filter(|item| {
                item.player_from_id == player.id
                    && item.player_from_team != item.player_to_team
                    && item.player_from_id != item.player_to_id
            })
            .map(|item| (item.player_to_id, item.distance))
    }
}

impl Eq for PlayerDistanceItem {}

impl PartialEq<PlayerDistanceItem> for PlayerDistanceItem {
    fn eq(&self, other: &Self) -> bool {
        self.player_from_id == other.player_from_id
            && self.player_from_team == other.player_from_team
            && self.player_to_id == other.player_to_id
            && self.player_to_team == other.player_to_team
            && self.distance == other.distance
    }
}

impl PartialOrd<Self> for PlayerDistanceItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerDistanceItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}
