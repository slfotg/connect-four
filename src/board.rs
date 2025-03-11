#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Column {
    pub const ALL: [Column; 7] = [
        Column::A,
        Column::B,
        Column::C,
        Column::D,
        Column::E,
        Column::F,
        Column::G,
    ];
}

impl std::convert::TryFrom<&str> for Column {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "A" => Ok(Column::A),
            "B" => Ok(Column::B),
            "C" => Ok(Column::C),
            "D" => Ok(Column::D),
            "E" => Ok(Column::E),
            "F" => Ok(Column::F),
            "G" => Ok(Column::G),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for Column {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

impl std::ops::Not for Player {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Board {
    pub red: u64,
    pub yellow: u64,
}

impl Board {
    pub const ROWS: usize = 6;
    pub const COLS: usize = 7;
    const EMPTY_CELL: Cell = Cell::Empty;
    const RED_CELL: Cell = Cell::Occupied(Player::Red);
    const YELLOW_CELL: Cell = Cell::Occupied(Player::Yellow);

    pub fn is_possible(&self, c: &Column) -> bool {
        self[(Board::ROWS - 1, *c)] == Self::EMPTY_CELL
    }

    pub fn possible_moves(&self) -> Vec<Column> {
        Column::ALL
            .iter()
            .filter(|c| self.is_possible(c))
            .copied()
            .collect()
    }

    pub fn apply_move(&self, c: Column, p: Player) -> (Self, usize) {
        let mut board = *self;
        for r in 0..Board::ROWS {
            let mask = 1 << (r * Board::COLS + c as usize);
            if mask & (self.red | self.yellow) != mask {
                match p {
                    Player::Red => board.red |= mask,
                    Player::Yellow => board.yellow |= mask,
                }
                return (board, r);
            }
        }
        panic!("Invalid move");
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = Cell;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        let mask = 1 << (r * Self::COLS + c);
        if self.red & mask > 0 {
            &Self::RED_CELL
        } else if self.yellow & mask > 0 {
            &Self::YELLOW_CELL
        } else {
            &Self::EMPTY_CELL
        }
    }
}

impl std::ops::Index<(usize, Column)> for Board {
    type Output = Cell;
    fn index(&self, (r, c): (usize, Column)) -> &Self::Output {
        self.index((r, c as usize))
    }
}
