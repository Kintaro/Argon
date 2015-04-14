use board::position::Position;
use board::color::StoneColor;

pub enum Move {
    /// A normal playing move with position and color
    Play(Position, StoneColor),
    /// A player passed
    Pass(StoneColor),
    /// A player resigned
    Resign(StoneColor)
}

impl Move {
    /// Returns the position if the move is a play,
    /// returns None otherwise.
    pub fn position(&self) -> Option<Position> {
        match self {
            &Move::Play(ref pos, _) => Some(pos.clone()),
            _                       => None
        }
    }

    /// Retrieves the move's color
    pub fn color(&self) -> &StoneColor {
        match self {
            &Move::Play(_, ref color) => color,
            &Move::Pass(ref color)    => color,
            &Move::Resign(ref color)  => color
        }
    }

    pub fn is_play(&self) -> bool {
        match self {
            &Move::Play(_, _) => true,
            _           => false,
        }
    }

    pub fn is_pass(&self) -> bool {
        match self {
            &Move::Pass(_) => true,
            _        => false,
        }
    }

    pub fn is_resign(&self) -> bool {
        match self {
            &Move::Resign(_) => true,
            _          => false
        }
    }
}
