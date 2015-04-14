pub enum StoneColor {
    Black, 
    White
}

impl StoneColor {
    /// Create a new stonecolor from a given string. If the string cannot
    /// be parsed, return None.
    /// This is mostly used to interface with GTP.
    pub fn from_string(string: &str) -> Option<StoneColor> {
        match string {
            "b" | "B" => Some(StoneColor::Black),
            "w" | "W" => Some(StoneColor::White),
            _         => None
        }
    }

    /// Get the opposite color.
    pub fn opposite_color(&self) -> StoneColor {
        match self {
            &StoneColor::Black => StoneColor::White,
            _                  => StoneColor::Black
        }
    }
}
