use crate::board::Column;
use crate::state::GameState;

use super::Agent;

#[derive(Default)]
pub struct CliAgent {}

impl Agent for CliAgent {
    fn next_move(&self, state: &GameState) -> Column {
        let valid_moves = state.possible_moves();
        let mut input = String::new();
        let mut col = None;
        while col.is_none() {
            println!("Enter column: ");
            std::io::stdin().read_line(&mut input).unwrap();
            col = input.trim().parse::<Column>().ok().and_then(|c| {
                if !valid_moves.contains(&c) {
                    println!("Invalid move");
                    None
                } else {
                    Some(c)
                }
            });
        }
        col.unwrap()
    }
}
