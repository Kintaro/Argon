use board::color::StoneColor;

#[derive(Clone, PartialEq, Eq)]
pub enum Intersection {
    Stone(StoneColor),
    Empty
}

impl Intersection {
    /// Creates a new intersection point of the given color.
    pub fn from_stone_color(color: StoneColor) -> Intersection {
        Intersection::Stone(color)
    }

    /// Returns the opposite color if the intersection is non-empty.
    pub fn opposite_color(&self) -> Intersection {
        match self {
            &Intersection::Stone(ref x) => Intersection::from_stone_color(x.opposite_color()),
            _                           => Intersection::Empty
        }
    }

    /// Check if the intersection is empty
    pub fn is_empty(&self) -> bool {
        match self {
            &Intersection::Empty => true,
            _                    => false
        }
    }
}
