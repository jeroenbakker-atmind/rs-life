use std::ops::{Add, Mul};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    pub fn offset(&self, x: isize, y: isize) -> Position {
        Position::new(self.x + x, self.y + y)
    }

    pub fn neighbours(&self) -> [Position; 8] {
        [
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x, self.y - 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x - 1, self.y + 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, self.y + 1),
        ]
    }
}

impl Mul<Position> for Position {
    type Output = Position;

    fn mul(self, rhs: Position) -> Position {
        Position::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}
