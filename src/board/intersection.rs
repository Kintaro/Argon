pub enum Intersection {
    Stone(StoneColor),
    Empty
}

impl Intersection {
    /// Creates a new intersection point of the given color.
    pub fn from_stone_color(color: StoneColor) -> Intersection {
        Stone(color)
    }

    /// Returns the opposite color if the intersection is non-empty.
    pub fn opposite_color(&self) -> Intersection {
        match self {
            &Stone(ref x) => Intersection::from_stone_color(x.opposite_color()),
            _             => Empty
        }
    }

    /// Check if the intersection is empty
    pub fn is_empty(&self) -> bool {
        match self {
            &Empty => true,
            _      => false
        }
    }
}
