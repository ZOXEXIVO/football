use crate::club::team::behaviour::TeamBehaviourResult;
use crate::club::PlayerCollectionResult;
use crate::shared::{Currency, CurrencyValue};
use crate::simulator::SimulatorData;
use crate::{StaffCollectionResult, TeamTrainingResult};

pub struct TeamResult {
    pub team_id: u32,
    pub players: PlayerCollectionResult,
    pub staffs: StaffCollectionResult,
    pub behaviour: TeamBehaviourResult,
    pub training: TeamTrainingResult,
}

impl TeamResult {
    pub fn new(
        team_id: u32,
        players: PlayerCollectionResult,
        staffs: StaffCollectionResult,
        behaviour: TeamBehaviourResult,
        training: TeamTrainingResult,
    ) -> Self {
        TeamResult {
            team_id,
            players,
            staffs,
            behaviour,
            training,
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        let team = data.team_mut(self.team_id).unwrap();

        for player_result in &self.players.players {
            team.add_player_to_transfer_list(
                player_result.player_id,
                CurrencyValue {
                    amount: 100000 as f64,
                    currency: Currency::Usd,
                },
            )
        }

        self.players.process(data);
        self.staffs.process(data);

        self.training.process(data);
    }
}
