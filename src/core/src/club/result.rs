use crate::club::academy::result::ClubAcademyResult;
use crate::club::{BoardResult, ClubFinanceResult};
use crate::simulator::SimulatorData;
use crate::{
    PlayerContractProposal, PlayerMessage, PlayerMessageType, PlayerResult, SimulationResult,
    TeamResult,
};

pub struct ClubResult {
    pub finance: ClubFinanceResult,
    pub teams: Vec<TeamResult>,
    pub board: BoardResult,
    pub academy: ClubAcademyResult,
}

impl ClubResult {
    pub fn new(
        finance: ClubFinanceResult,
        teams: Vec<TeamResult>,
        board: BoardResult,
        academy: ClubAcademyResult,
    ) -> Self {
        ClubResult {
            finance,
            teams,
            board,
            academy,
        }
    }

    pub fn process(self, data: &mut SimulatorData, _result: &mut SimulationResult) {
        self.finance.process(data);

        for team_result in &self.teams {
            for player_result in &team_result.players.players {
                if player_result.has_contract_actions() {
                    Self::process_player_contract_interaction(player_result, data);
                }
            }

            team_result.process(data);
        }

        self.board.process(data);
        self.academy.process(data);
    }

    fn process_player_contract_interaction(result: &PlayerResult, data: &mut SimulatorData) {
        if result.contract.no_contract || result.contract.want_improve_contract {
            let player = data.player(result.player_id).unwrap();

            let player_growth_potential = player.growth_potential(data.date.date());

            player.mailbox.push(PlayerMessage {
                message_type: PlayerMessageType::ContractProposal(PlayerContractProposal {
                    salary: get_contract_salary(player_growth_potential),
                    years: 3,
                }),
            })
        }

        fn get_contract_salary(player_growth_potential: u8) -> u32 {
            match player_growth_potential as u32 {
                0..=3 => 1000u32,
                4 => 2000u32,
                5 => 3000u32,
                _ => 1000u32,
            }
        }
    }
}
