use crate::board::Board;
use crate::board::Cell;
use crate::board::Player;
use ansi_term::Color;
use ansi_term::Style;
use itertools;

pub struct BoardAnsiWriter(pub Board);

impl BoardAnsiWriter {
    const MISSING: Color = Color::Fixed(235);
    const BLUE: Color = Color::Fixed(27);
    const RED: Color = Color::Fixed(196);
    const YELLOW: Color = Color::Fixed(226);
    const BACKGROUND: Style = Style {
        foreground: Some(Self::MISSING),
        background: Some(Self::BLUE),
        is_bold: false,
        is_dimmed: false,
        is_italic: false,
        is_underline: false,
        is_blink: false,
        is_reverse: false,
        is_hidden: false,
        is_strikethrough: false,
    };
}

impl std::fmt::Display for BoardAnsiWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(target_os = "windows")]
        ansi_term::enable_ansi_support().expect("ANSI colors not supported");

        let space = Self::BACKGROUND.paint(" ").to_string();
        let empty = Self::BACKGROUND.paint("●").to_string();
        let red = Self::BACKGROUND.fg(Self::RED).paint("◉").to_string();
        let yellow = Self::BACKGROUND.fg(Self::YELLOW).paint("◉").to_string();

        writeln!(f, "   A B C D E F G ").unwrap();
        let Self(board) = self;
        ((0..Board::ROWS).rev()).try_for_each(|r| {
            let line: String = itertools::intersperse(
                (0..Board::COLS).map(|c| match board[(r, c)] {
                    Cell::Occupied(Player::Red) => red.clone(),
                    Cell::Occupied(Player::Yellow) => yellow.clone(),
                    Cell::Empty => empty.clone(),
                }),
                space.clone(),
            )
            .collect();
            writeln!(f, "  {}{}{}", space, line, space)
        })
    }
}
