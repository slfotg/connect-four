use crate::board::Board;

const SIZE: usize = Board::ROWS * (Board::COLS - 3)
    + (Board::ROWS - 3) * Board::COLS
    + 2 * (Board::ROWS - 3) * (Board::COLS - 3);
pub const CONNECT_FOURS: [u64; SIZE] = calc_connect_4s();

pub const CONNECT_FOUR_LOOKUP: [[u128; Board::COLS]; Board::ROWS] = calc_connect_four_lookup();

const fn calc_connect_4s() -> [u64; SIZE] {
    let mut connect_4s = [0; SIZE];
    let mut i = 0;
    let mut r = 0;
    while r < Board::ROWS {
        let mut c = 0;
        while c < Board::COLS {
            let m = 1 << (r * Board::COLS + c);
            if c + 3 < Board::COLS {
                connect_4s[i] = m | m << 1 | m << 2 | m << 3;
                i += 1;
            }
            if r + 3 < Board::ROWS {
                connect_4s[i] =
                    m | m << Board::COLS | m << (Board::COLS * 2) | m << (Board::COLS * 3);
                i += 1;
                if c + 3 < Board::COLS {
                    connect_4s[i] = m
                        | m << (Board::COLS + 1)
                        | m << ((Board::COLS * 2) + 2)
                        | m << ((Board::COLS * 3) + 3);
                    i += 1;
                }
                if c >= 3 {
                    connect_4s[i] = m
                        | m << (Board::COLS - 1)
                        | m << (Board::COLS * 2 - 2)
                        | m << (Board::COLS * 3 - 3);
                    i += 1;
                }
            }
            c += 1;
        }
        r += 1;
    }
    connect_4s
}

const fn calc_connect_four_lookup() -> [[u128; Board::COLS]; Board::ROWS] {
    let mut connect_four_lookup = [[0; Board::COLS]; Board::ROWS];
    let mut r = 0;
    while r < Board::ROWS {
        let cell: u64 = 1 << (r * Board::COLS);
        let mut c = 0;
        while c < Board::COLS {
            let cell = cell << c;
            let mut i = 0;
            while i < CONNECT_FOURS.len() {
                // TODO: Implement this
                if cell & CONNECT_FOURS[i] == cell {
                    connect_four_lookup[r][c] |= 1 << i;
                }
                i += 1;
            }
            c += 1;
        }
        r += 1;
    }
    connect_four_lookup
}
