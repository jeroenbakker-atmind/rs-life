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
    pub fn print_grid(&self) {
        let min_x = self.alive_cells.iter().map(|p| p.x).min().unwrap_or(0);
        let max_x = self.alive_cells.iter().map(|p| p.x).max().unwrap_or(0);
        let min_y = self.alive_cells.iter().map(|p| p.y).min().unwrap_or(0);
        let max_y = self.alive_cells.iter().map(|p| p.y).max().unwrap_or(0);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.is_alive(Position::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
