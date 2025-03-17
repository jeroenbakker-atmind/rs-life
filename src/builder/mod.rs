use crate::life::{grid::Grid, position::Position};

pub trait Builder {
    fn cell(&mut self, position: Position) -> &mut Self;
    fn block(&mut self, position: Position) -> &mut Self;
}

#[derive(Copy, Clone)]
pub enum Orientation {
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

impl Orientation {
    fn as_multiplier(self) -> Position {
        match self {
            Orientation::NorthEast => Position::new(1, 1),
            Orientation::SouthEast => Position::new(1, -1),
            Orientation::NorthWest => Position::new(-1, 1),
            Orientation::SouthWest => Position::new(-1, -1),
        }
    }
}

impl Builder for Grid {
    fn cell(&mut self, position: Position) -> &mut Self {
        self.add_cell(position);
        self
    }
    fn block(&mut self, position: Position) -> &mut Self {
        self.cell(position)
            .cell(position.offset(1, 0))
            .cell(position.offset(0, 1))
            .cell(position.offset(1, 1))
    }
}
