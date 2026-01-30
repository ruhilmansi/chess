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

        self.squares[0][0] = Some(Piece::new(Rook, White));
        self.squares[0][1] = Some(Piece::new(Knight, White));
        self.squares[0][2] = Some(Piece::new(Bishop, White));
        self.squares[0][3] = Some(Piece::new(Queen, White));
        self.squares[0][4] = Some(Piece::new(King, White));
        self.squares[0][5] = Some(Piece::new(Bishop, White));
        self.squares[0][6] = Some(Piece::new(Knight, White));
        self.squares[0][7] = Some(Piece::new(Rook, White));

        for i in 0..8 {
            self.squares[1][i] = Some(Piece::new(Pawn, White));
        }

        self.squares[7][0] = Some(Piece::new(Rook, Black));
        self.squares[7][1] = Some(Piece::new(Knight, Black));
        self.squares[7][2] = Some(Piece::new(Bishop, Black));
        self.squares[7][3] = Some(Piece::new(Queen, Black));
        self.squares[7][4] = Some(Piece::new(King, Black));
        self.squares[7][5] = Some(Piece::new(Bishop, Black));
        self.squares[7][6] = Some(Piece::new(Knight, Black));
        self.squares[7][7] = Some(Piece::new(Rook, Black));

        for i in 0..8 {
            self.squares[6][i] = Some(Piece::new(Pawn, Black));
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<Piece> {
        self.squares[r][c]
    }

    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), color: Color) -> bool {
        let from_piece = match self.get(from.0, from.1) {
            Some(p) => p,
            None => return false,
        };

        if from_piece.color != color {
            return false;
        }

        if from == to {
            return false;
        }

        if let Some(target) = self.get(to.0, to.1) {
            if target.color == color {
                return false;
            }
        }

        match from_piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(from, to, color),
            PieceType::Knight => self.is_valid_knight_move(from, to),
            PieceType::Bishop => self.is_valid_bishop_move(from, to),
            PieceType::Rook => self.is_valid_rook_move(from, to),
            PieceType::Queen => self.is_valid_queen_move(from, to),
            PieceType::King => self.is_valid_king_move(from, to),
        }
    }

    fn is_valid_pawn_move(&self, from: (usize, usize), to: (usize, usize), color: Color) -> bool {
        let direction = match color {
            Color::White => 1i32,  
            Color::Black => -1i32, 
        };

        let start_row = match color {
            Color::White => 1,
            Color::Black => 6,
        };

        let from_r = from.0 as i32;
        let from_c = from.1 as i32;
        let to_r = to.0 as i32;
        let to_c = to.1 as i32;

        if to_c == from_c && to_r == from_r + direction {
            return self.get(to.0, to.1).is_none();
        }

        if from.0 == start_row && to_c == from_c && to_r == from_r + 2 * direction {
            let mid_r = (from_r + direction) as usize;
            return self.get(mid_r, from.1).is_none() && self.get(to.0, to.1).is_none();
        }

        if (to_c - from_c).abs() == 1 && to_r == from_r + direction {
            return self.get(to.0, to.1).is_some();
        }

        false
    }

    fn is_valid_knight_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let dr = (from.0 as i32 - to.0 as i32).abs();
        let dc = (from.1 as i32 - to.1 as i32).abs();
        (dr == 2 && dc == 1) || (dr == 1 && dc == 2)
    }

    fn is_valid_bishop_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let dr = (from.0 as i32 - to.0 as i32).abs();
        let dc = (from.1 as i32 - to.1 as i32).abs();

        if dr != dc {
            return false;
        }

        self.is_path_clear(from, to)
    }

    fn is_valid_rook_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if from.0 != to.0 && from.1 != to.1 {
            return false;
        }

        self.is_path_clear(from, to)
    }

    fn is_valid_queen_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        self.is_valid_rook_move(from, to) || self.is_valid_bishop_move(from, to)
    }

    fn is_valid_king_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let dr = (from.0 as i32 - to.0 as i32).abs();
        let dc = (from.1 as i32 - to.1 as i32).abs();
        dr <= 1 && dc <= 1
    }

    fn is_path_clear(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let dr = (to.0 as i32 - from.0 as i32).signum();
        let dc = (to.1 as i32 - from.1 as i32).signum();

        let mut r = from.0 as i32 + dr;
        let mut c = from.1 as i32 + dc;

        while (r, c) != (to.0 as i32, to.1 as i32) {
            if self.get(r as usize, c as usize).is_some() {
                return false;
            }
            r += dr;
            c += dc;
        }

        true
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
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
            return true;
        }
        false
    }

    pub fn get_white_points(&self) -> u32 {
        self.captured_black.iter().map(|p| p.point_value()).sum()
    }

    pub fn get_black_points(&self) -> u32 {
        self.captured_white.iter().map(|p| p.point_value()).sum()
    }
}
