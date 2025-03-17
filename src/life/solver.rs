use std::collections::HashSet;

use super::{grid::Grid, position::Position};

pub fn next_generation(grid: &Grid) -> Grid {
    let mut result = Grid::new();
    for position in positions_to_evaluate(grid) {
        let is_alive = grid.is_alive(position);
        let num_living_neighbours = position
            .neighbours()
            .iter()
            .filter(|neighbor_position| grid.is_alive(**neighbor_position))
            .count();
        if apply_rules(is_alive, num_living_neighbours) {
            result.add_cell(position)
        }
    }
    result
}

fn positions_to_evaluate(grid: &Grid) -> HashSet<Position> {
    let mut result = HashSet::<Position>::default();
    for position in &grid.alive_cells {
        result.insert(*position);
        for neighbour in position.neighbours() {
            result.insert(neighbour);
        }
    }
    result
}

/// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// - Any live cell with two or three live neighbours lives on to the next generation.
/// - Any live cell with more than three live neighbours dies, as if by overpopulation.
/// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
pub fn apply_rules(is_alive: bool, num_living_neighbours: usize) -> bool {
    if num_living_neighbours == 3 {
        true
    } else if num_living_neighbours == 2 {
        is_alive
    } else {
        false
    }
}
