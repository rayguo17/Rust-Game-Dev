use crate::prelude::*;


#[system]
pub fn end_turn(
    #[resource] current_state: &mut TurnState
){
    let new_state = match current_state {
        TurnState::AwaitPlayerInput=>return,
        TurnState::PlayerMove=>TurnState::MonsterMove,
        TurnState::MonsterMove=>TurnState::AwaitPlayerInput,
        
    };
    *current_state = new_state;
}