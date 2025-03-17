use crate::life::{grid::Grid, position::Position};

/// A trait for building patterns on a grid.
pub trait Builder {
    /// Adds a single cell at the specified position on the grid.
    ///
    /// # Arguments
    ///
    /// * `position` - The position where the cell should be added.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder, allowing for method chaining.
    fn cell(&mut self, position: Position) -> &mut Self;

    /// Adds a 2x2 block of cells with the top-left corner at the specified position.
    ///
    /// # Arguments
    ///
    /// * `position` - The top-left corner position of the block.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder, allowing for method chaining.
    fn block(&mut self, position: Position) -> &mut Self;

    /// Adds a glider pattern centered around the specified position and oriented according to the specified orientation.
    ///
    /// # Arguments
    ///
    /// * `position` - The center position of the glider.
    /// * `orientation` - The orientation of the glider.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder, allowing for method chaining.
    fn glider(&mut self, position: Position, orientation: Orientation) -> &mut Self;
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

    fn glider(&mut self, position: Position, orientation: Orientation) -> &mut Self {
        let multiplier = orientation.as_multiplier();
        self.cell(position.offset(-1, -1) * multiplier)
            .cell(position.offset(0, -1) * multiplier)
            .cell(position.offset(1, -1) * multiplier)
            .cell(position.offset(1, 0) * multiplier)
            .cell(position.offset(0, 1) * multiplier)
    }
}
