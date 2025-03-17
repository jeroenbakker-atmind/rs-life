//! A grid of cells.

use std::collections::HashSet;

use super::position::Position;

pub struct Grid {
    pub alive_cells: HashSet<Position>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            alive_cells: HashSet::default(),
        }
    }
    pub fn add_cell(&mut self, position: Position) {
        self.alive_cells.insert(position);
    }
    pub fn is_alive(&self, position: Position) -> bool {
        self.alive_cells.contains(&position)
    }
}
