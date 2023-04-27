// TODO: will also be important for implementing the 50 move rule
// TODO: the 3 fold repetition rule and insufficient material rule

use crate::core::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Modifiers {
    pub can_white_castle_kingside: bool,
    pub can_white_castle_queenside: bool,
    pub can_black_castle_kingside: bool,
    pub can_black_castle_queenside: bool,
    pub en_passant: Option<Position>,
    pub promotion: Option<Position>,
    pub halfmove_clock: u8,
    // pub repetition_count: u8,
    // pub insufficient_material: bool,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers {
            can_white_castle_kingside: true,
            can_white_castle_queenside: true,
            can_black_castle_kingside: true,
            can_black_castle_queenside: true,
            en_passant: None,
            promotion: None,
            halfmove_clock: 0,
        }
    }
}
