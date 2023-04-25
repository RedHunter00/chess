#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}


impl std::fmt::Display for Pieces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pieces::Pawn => write!(f, "P"),
            Pieces::Knight => write!(f, "Kn"),
            Pieces::Bishop => write!(f, "B"),
            Pieces::Rook => write!(f, "R"),
            Pieces::Queen => write!(f, "Q"),
            Pieces::King => write!(f, "K"),
        }
    }
}
