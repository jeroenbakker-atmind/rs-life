use rs_life::{
    builder::{Builder, Orientation},
    life::{grid::Grid, position::Position, solver::next_generation},
};

fn main() {
    let mut grid = Grid::new();

    grid.glider(Position::new(10, -13), Orientation::SouthEast)
        .glider(Position::new(1, -4), Orientation::SouthEast)
        .bumper_p4(Position::new(0, 0), Orientation::NorthEast);

    for i in 0..100 {
        println!(" --- Generation {i} ---");
        grid.print_grid();
        println!("");
        println!("");
        grid = next_generation(&grid)
    }
}
