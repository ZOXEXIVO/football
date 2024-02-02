use crate::r#match::{MatchContext, MatchField, MatchState, PlayMatchStateResult};

pub struct StateManager {
    current_state: MatchState,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current_state: MatchState::FirstHalf,
        }
    }

    pub fn current(&self) -> MatchState {
        self.current_state
    }

    pub fn next(&mut self) -> Option<MatchState> {
        let next_state = Self::get_next_state(self.current_state);

        match next_state {
            MatchState::End => None,
            _ => {
                self.current_state = next_state;
                Some(self.current_state)
            }
        }
    }

    fn get_next_state(current_state: MatchState) -> MatchState {
        match current_state {
            MatchState::Initial => MatchState::FirstHalf,
            MatchState::FirstHalf => MatchState::HalfTime,
            MatchState::HalfTime => MatchState::SecondHalf,
            MatchState::SecondHalf => MatchState::ExtraTime,
            MatchState::ExtraTime => MatchState::PenaltyShootout,
            MatchState::PenaltyShootout => MatchState::End,
            MatchState::End => MatchState::End,
        }
    }

    pub fn handle_state_finish(context: &mut MatchContext, field: &mut MatchField, play_result: PlayMatchStateResult) {
        if context.state.match_state.need_swap_squads() {
            field.swap_squads();
        }

        if play_result.additional_time > 0 {
            context.add_time(play_result.additional_time);
        }

        match context.state.match_state {
            MatchState::Initial => {}
            MatchState::FirstHalf => {
                Self::play_rest_time(field);
            }
            MatchState::HalfTime => {}
            MatchState::SecondHalf => {}
            MatchState::ExtraTime => {}
            MatchState::PenaltyShootout => {}
            _ => {}
        }
    }

    fn play_rest_time(field: &mut MatchField) {
        field.players.iter_mut().for_each(|p| {
            p.player_attributes.rest(1000);
        })
    }
}
