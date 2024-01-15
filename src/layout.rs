use std::collections::HashMap;


pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    LeftThumb,
    RightThumb,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

/// A key on the keyboard.
/// Has a position and a finger.
pub struct Key {
    finger: Finger,
    position: (f64, f64),
}

pub struct Layout {
    mapping: HashMap<char, Key>,
}
