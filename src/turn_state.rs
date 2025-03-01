pub use crate::prelude::*;

#[derive(Clone,PartialEq,Debug)]
pub enum TurnState{
    AwaitPlayerInput,
    PlayerMove,
    MonsterMove,
}


