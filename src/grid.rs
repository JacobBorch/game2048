#[derive(Debug, PartialEq)]
pub struct Grid {
    // 4x4 grid
    cells: [[u64; 4]; 4],
}

impl Grid {
    fn new(cells: [[u64; 4]; 4]) -> Self {
        Self { cells }
    }

    fn mov(&mut self, mov: Move) {
        // Implementation of Going right.
        for j in 0..4 {
            let old_row = self.cells[j];
            let mut new_row = self.cells[j];

            // SPECIAL CASE: [x, x, x, x] => [0, 0, 2x, 2x] (all are same)
            let val = old_row[0];
            if val != 0 && old_row.iter().all(|x| *x == val) {
                let new_val = 2 * val;
                new_row = [0, 0, new_val, new_val];
                self.cells[j] = new_row;
                continue;
            }

            for i in 0..3 {
                // [2, 0] => [0, 2]
                if new_row[i + 1] == 0 {
                    new_row.swap(i, i + 1);
                }
                // [2, 2] => [0, 4]
                else if old_row[i + 1] == old_row [i] {
                    new_row[i] = 0;
                    new_row[i + 1] *= 2;
                }
            }
            // [0, 2, 0, 2] => [0, 0, 2, 2]
            for i in (1..=3).rev() {
                if new_row[i] == 0 {
                    new_row.swap(i, i - 1);
                } 
            }

            self.cells[j] = new_row;
        }
    }
}

enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::{Grid, Move};

    #[test]
    fn move_right_works() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov(Move::Right);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_right_works2() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov(Move::Right);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_combination_works() {
        let row1 = [2, 2, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov(Move::Right);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn cell_cant_combine_more_than_once() {
        let row1 = [2, 2, 4, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov(Move::Right);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 4, 8];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov(Move::Right);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn doesnt_double_combinate_when_all_are_the_same() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 2];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov(Move::Right);
        assert_eq!(grid, result_grid)
    }
}
