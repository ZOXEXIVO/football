use crate::league::Season;

#[derive(Debug)]
pub struct PlayerStatistics {
    pub played: u16,
    pub played_subs: u16,

    pub goals: u16,
    pub assists: u16,
    pub penalties: u16,
    pub player_of_the_match: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,

    pub shots_on_target: f32,
    pub tackling: f32,
    pub passes: u8,

    pub average_rating: f32,
}

impl PlayerStatistics {
    pub fn new() -> Self {
        PlayerStatistics {
            played: 0,
            played_subs: 0,
            goals: 0,
            assists: 0,
            penalties: 0,
            player_of_the_match: 0,
            yellow_cards: 0,
            red_cards: 0,
            shots_on_target: 0.0,
            tackling: 0.0,
            passes: 0,
            average_rating: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct PlayerStatisticsHistory {
    pub items: Vec<PlayerStatisticsHistoryItem>,
}

#[derive(Debug)]
pub struct PlayerStatisticsHistoryItem {
    pub season: Season,
    pub statistics: PlayerStatistics,
}

impl PlayerStatisticsHistory {
    pub fn new() -> Self {
        PlayerStatisticsHistory { items: Vec::new() }
    }
}
