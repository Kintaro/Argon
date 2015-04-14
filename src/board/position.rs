
/// Represents a position on the board.
/// It starts counting from 1, so that 0 can be used
/// to represent the border.
#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    row: u8,
    column: u8
}

impl Position {
    /// Create a new board position with the given row and column
    pub fn new(r: u8, c: u8) -> Position {
        Position { row: r, column: c }
    }

    /// Generate all positions on the board
    /// (excluding the border)
    pub fn generate_positions(board_size: u8) -> Vec<Position> {
        (0us..(board_size as usize * board_size as usize))
            .map(|index| Position::from_linear_index(board_size, index))
            .collect()
    }

    /// Creates a new position from a linear index on the board, mapping
    /// the point to [size * row + column]
    pub fn from_linear_index(board_size: u8, index: usize) -> Position {
        Position::new((index / board_size as usize + 1) as u8,
            (index % board_size as usize + 1) as u8) 
    }

    /// Maps the position back to its linear index
    pub fn to_linear_index(&self, board_size: u8) -> usize {
        board_size as usize * (self.row as usize - 1) + (self.column as usize - 1)
    }

    /// Checks if the position is a border point
    /// and thus to be ignored in many cases
    pub fn is_border(&self) -> bool {
        self.row == 0 || self.column == 0
    }

    /// Checks if the position is inside the board
    /// and not a border point.
    pub fn is_inside(&self) -> bool {
        !self.is_border()
    }
}
