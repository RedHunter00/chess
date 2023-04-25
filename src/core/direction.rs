/// Core enum that represents the direction of a move
/// Used in the Position struct to get a relative position from a given position
/// in a specific direction
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    RightRightUp,
    RightRightDown,
    LeftLeftUp,
    LeftLeftDown,
    UpUpRight,
    UpUpLeft,
    DownDownRight,
    DownDownLeft,
}
