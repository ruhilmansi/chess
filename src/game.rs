use crate::{board::Board, pieces::Color};

pub struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
        }
    }

    pub fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
}
