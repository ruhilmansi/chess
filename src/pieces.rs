#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn to_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => '♙',
            (PieceType::Knight, Color::White) => '♘',
            (PieceType::Bishop, Color::White) => '♗',
            (PieceType::Rook, Color::White) => '♖',
            (PieceType::Queen, Color::White) => '♕',
            (PieceType::King, Color::White) => '♔',
            (PieceType::Pawn, Color::Black) => '♟',
            (PieceType::Knight, Color::Black) => '♞',
            (PieceType::Bishop, Color::Black) => '♝',
            (PieceType::Rook, Color::Black) => '♜',
            (PieceType::Queen, Color::Black) => '♛',
            (PieceType::King, Color::Black) => '♚',
        }
    }

    pub fn point_value(&self) -> u32 {
        match self.piece_type {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }
}
