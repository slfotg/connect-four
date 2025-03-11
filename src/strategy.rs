use crate::board::Column;
use crate::state::GameState;

pub mod cli;
pub mod mcts;
pub mod random;

pub trait Agent {
    fn next_move(&self, board: &GameState) -> Column;
}
