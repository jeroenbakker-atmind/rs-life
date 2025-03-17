#[cfg(test)]
mod tests {
    use crate::life::grid::Grid;
    use crate::life::position::Position;
    use crate::life::solver::{apply_rules, next_generation};

    #[test]
    fn test_apply_rules() {
        assert_eq!(apply_rules(true, 0), false);
        assert_eq!(apply_rules(true, 1), false);
        assert_eq!(apply_rules(true, 2), true);
        assert_eq!(apply_rules(true, 3), true);
        assert_eq!(apply_rules(true, 4), false);
        assert_eq!(apply_rules(false, 3), true);
        assert_eq!(apply_rules(false, 2), false);
    }

    #[test]
    fn test_next_generation() {
        let mut grid = Grid::new();
        grid.add_cell(Position::new(1, 0));
        grid.add_cell(Position::new(1, 1));
        grid.add_cell(Position::new(1, 2));

        let next_gen = next_generation(&grid);

        assert!(next_gen.is_alive(Position::new(0, 1)));
        assert!(next_gen.is_alive(Position::new(1, 1)));
        assert!(next_gen.is_alive(Position::new(2, 1)));
        assert!(!next_gen.is_alive(Position::new(1, 0)));
        assert!(!next_gen.is_alive(Position::new(1, 2)));
    }
}
