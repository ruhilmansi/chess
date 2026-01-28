use crate::pieces::{Color, Piece, PieceType};
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub captured_white: Vec<Piece>,
    pub captured_black: Vec<Piece>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            squares: [[None; 8]; 8],
            captured_white: vec![],
            captured_black: vec![],
        };
        board.setup();
        board
    }

    fn setup(&mut self) {
        use PieceType::*;
        use Color::*;

        let back = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

        for i in 0..8 {
            self.squares[1][i] = Some(Piece::new(Pawn, White));
            self.squares[6][i] = Some(Piece::new(Pawn, Black));
            self.squares[0][i] = Some(Piece::new(back[i], White));
            self.squares[7][i] = Some(Piece::new(back[i], Black));
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<Piece> {
        self.squares[r][c]
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        if let Some(p) = self.squares[from.0][from.1] {
            if let Some(captured) = self.squares[to.0][to.1] {
                if captured.color == Color::White {
                    self.captured_white.push(captured);
                } else {
                    self.captured_black.push(captured);
                }
            }

            self.squares[to.0][to.1] = Some(p);
            self.squares[from.0][from.1] = None;
        }
    }
}
