// ! not used yet
// & will be important for customizing the rules of the game

pub struct Rules {
    is_white_allowed_to_castle_kingside: bool,
    is_white_allowed_to_castle_queenside: bool,
    is_black_allowed_to_castle_kingside: bool,
    is_black_allowed_to_castle_queenside: bool,
    is_en_passant_allowed: bool,
    is_promotion_to_queen_allowed: bool,
    is_promotion_to_rook_allowed: bool,
    is_promotion_to_bishop_allowed: bool,
    is_promotion_to_knight_allowed: bool,
    is_halfmove_clock_allowed: bool,
}