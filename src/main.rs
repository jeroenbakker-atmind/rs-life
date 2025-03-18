use builder::Builder;
use life::{grid::Grid, position::Position, solver::next_generation};

mod builder;
mod life;

fn main() {
    let mut grid = Grid::new();

    // Use the glider function of the builder trait
    grid.glider(Position::new(0, 0), builder::Orientation::NorthEast);

    // Generate multiple generations
    for _ in 0..10 {
        grid = next_generation(&grid);
        grid.print_grid();
        println!("")
    }
}
