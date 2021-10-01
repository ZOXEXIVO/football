use crate::club::squad::SquadPlayer;
use crate::club::{Staff, PlayerPositionType, Player};
use crate::{Team};
use std::collections::HashSet;

pub struct PlayerSelector;

const DEFAULT_SQUAD_SIZE: usize = 11;
const DEFAULT_BENCH_SIZE: usize = 6;

const POSITIONS: &[PlayerPositionType; 8] = &[
    PlayerPositionType::Goalkeeper,
    PlayerPositionType::DefenderLeft,
    PlayerPositionType::DefenderCenter,
    PlayerPositionType::DefenderRight,
    PlayerPositionType::MidfielderLeft,
    PlayerPositionType::MidfielderCenter,
    PlayerPositionType::MidfielderRight,
    PlayerPositionType::Striker
];

pub struct PlayerSelectionResult<'s> {
    pub main_squad: Vec<SquadPlayer<'s>>,
    pub substitutes: Vec<SquadPlayer<'s>>
}

impl PlayerSelector {
    pub fn select<'c>(team: &'c Team, staff: &Staff) -> PlayerSelectionResult<'c> {
        let current_tactics = team.tactics.as_ref().unwrap();
        
        let mut main_squad: Vec<SquadPlayer<'c>> =
            Vec::with_capacity(DEFAULT_SQUAD_SIZE);

        let mut selected_players = HashSet::new();
                
        for player_position in current_tactics.positions() {
            for position_player in Self::select_by_type(team, player_position){                
                if staff.relations.is_favorite_player(position_player.player.id) {
                    main_squad.push(SquadPlayer::new(&position_player.player, *player_position))
                }
                else{
                    // TODO
                    main_squad.push(SquadPlayer::new(&position_player.player, *player_position))
                }
                
                selected_players.insert(position_player.player.id);
            }            
        }
        
        let mut substitutes: Vec<SquadPlayer<'c>> =
            Vec::with_capacity(DEFAULT_BENCH_SIZE);

        PlayerSelectionResult {
            main_squad,
            substitutes
        }
    }

    fn select_by_type<'c>(
        team: &'c Team,
        position: &PlayerPositionType,
    ) -> Vec<SquadPlayer<'c>> {
        let mut result: Vec<SquadPlayer<'c>> = Vec::with_capacity(3);
        
        let mut players_on_position: Vec<&Player> = team
            .players
            .players
            .iter()
            .filter(|p| p.positions().contains(position))
            .collect();

        players_on_position.sort_by(|a, b| {
            a.player_attributes.condition.cmp(&b.player_attributes.condition) 
        });

        result
    }
}