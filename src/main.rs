use life::{grid::Grid, solver::next_generation};

mod life;
mod builder;

fn main() {
    let grid = Grid::new();
    let new_generation = next_generation(&grid);
}
