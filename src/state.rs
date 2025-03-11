use crate::board::Board;
use crate::board::Column;
use crate::board::Player;
use crate::lookup;
use crate::lookup::CONNECT_FOURS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    InProgress,
    Draw,
    Win(Player),
}

impl State {
    pub fn is_over(&self) -> bool {
        match self {
            State::InProgress => false,
            State::Draw => true,
            State::Win(_) => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    possible_reds: u128,
    possible_yellows: u128,
    pub state: State,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: Board::default(),
            current_player: Player::Yellow,
            possible_reds: (1_u128 << CONNECT_FOURS.len()) - 1,
            possible_yellows: (1_u128 << CONNECT_FOURS.len()) - 1,
            state: State::InProgress,
        }
    }
}

impl GameState {
    pub fn possible_moves(&self) -> Vec<Column> {
        self.board.possible_moves()
    }

    pub fn apply_move(&mut self, c: Column) {
        let (board, row) = self.board.apply_move(c, self.current_player);
        self.board = board;

        // update other player's possible moves
        match self.current_player {
            Player::Yellow => self.possible_reds &= !lookup::CONNECT_FOUR_LOOKUP[row][c as usize],
            Player::Red => self.possible_yellows &= !lookup::CONNECT_FOUR_LOOKUP[row][c as usize],
        }

        // check for win
        let (mut possible, player_mask) = match self.current_player {
            Player::Yellow => (self.possible_yellows, self.board.yellow),
            Player::Red => (self.possible_reds, self.board.red),
        };
        possible &= lookup::CONNECT_FOUR_LOOKUP[row][c as usize];
        let mut index = 0;
        while possible > 0 {
            if possible & 1 == 1 {
                let connect_four = CONNECT_FOURS[index];
                if player_mask & connect_four == connect_four {
                    self.state = State::Win(self.current_player);
                    break;
                }
            }
            possible >>= 1;
            index += 1;
        }

        // check for draw
        if self.state == State::InProgress && self.possible_moves().is_empty() {
            self.state = State::Draw;
        }
        self.current_player = !self.current_player;
    }
}
