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

    /// Adds a bumper_p4 pattern at the specified position and oriented according to the specified orientation.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the bumper_p4.
    /// * `orientation` - The orientation of the bumper_p4.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder, allowing for method chaining.
    fn bumper_p4(&mut self, position: Position, orientation: Orientation) -> &mut Self;
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
        for offset in [(0, 0), (1, 0), (2, 1), (0, 1), (0, 2)] {
            self.cell(Position::new(offset.0, offset.1) * multiplier + position);
        }
        self
    }

    // TODO:
    // ....OO....OO............
    // ...O..O..O..X...........
    // ..........O.O...........
    // ...........O............
    // ..O..O..................
    // ..O.OO.........OO.......
    // ...O.OOO.......O........
    // OOO...O.O.......OOO.....
    // O.......O.........O.....
    // ........OO..............
    fn bumper_p4(&mut self, position: Position, orientation: Orientation) -> &mut Self {
        let multiplier = orientation.as_multiplier();
        for offset in [
            (-9, 0),
            (-8, -1),
            (-7, -1),
            (-6, 0),
            (0, 0),
            (-1, -1),
            (-2, -1),
            (-3, 0),
            (-2, 1),
            (-1, 2),
            (0, 1),
            (-10, 3),
            (-7, 3),
            (-10, 4),
            (-8, 4),
            (-7, 4),
            (3, 4),
            (4, 4),
            (-9, 5),
            (-7, 5),
            (-6, 5),
            (-5, 5),
            (3, 5),
            (-12, 6),
            (-11, 6),
            (-10, 6),
            (-6, 6),
            (-4, 6),
            (4, 6),
            (5, 6),
            (6, 6),
            (-12, 7),
            (-4, 7),
            (6, 7),
            (-4, 8),
            (-3, 8),
        ] {
            self.cell(Position::new(offset.0, offset.1) * multiplier + position);
        }
        self
    }
}
