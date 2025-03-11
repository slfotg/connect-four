use std::cell::RefCell;

use rand::prelude::*;

use super::Agent;

/// RandomAgent to select a random valid move given the current GameState
pub struct RandomAgent {
    pub rng: RefCell<ThreadRng>,
}

impl Default for RandomAgent {
    fn default() -> Self {
        Self {
            rng: RefCell::new(rand::rng()),
        }
    }
}

impl Agent for RandomAgent {
    fn next_move(&self, board: &crate::state::GameState) -> crate::board::Column {
        let moves = board.possible_moves();
        moves[self.rng.borrow_mut().random_range(0..moves.len())]
    }
}
