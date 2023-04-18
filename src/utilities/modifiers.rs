// ! not used yet
// & will be important for implementing the mechanics
// & of castleing, en passant, and promotion
// & will also be important for implementing the 50 move rule
// & the 3 fold repetition rule and insufficient material rule

use crate::positions::position::Position;

pub struct Modifiers {
    can_white_castle_kingside: bool,
    can_white_castle_queenside: bool,
    can_black_castle_kingside: bool,
    can_black_castle_queenside: bool,
    en_passant: Option<Position>,
    promotion: Option<Position>,
    halfmove_clock: usize,
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
