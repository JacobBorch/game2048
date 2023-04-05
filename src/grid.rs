#[derive(Debug, PartialEq)]
pub struct Grid {
    // 4x4 grid
    cells: [[u64; 4]; 4],
}

impl Grid {
    fn new(cells: [[u64; 4]; 4]) -> Self {
        Self { cells }
    }

    fn make_move(&mut self, mov: Move) {
        let rotation = mov.get_number();
    }

    fn handle_move(&mut self, rotation: usize) {

    }

    fn mov(&mut self) {
        // Implementation of Going right.
        self.mov_all_cells_to_the_side();

        for i in 0..3 {
            let old_row = self.cells[i];
            let mut new_row = old_row;

            //SPECIAL CASE: [x, x, x, x] => [0, 0, 2x, 2x]
            let val = old_row[0];
            if old_row.iter().all(|x| *x == val) {
                let new_val = 2 * val;
                self.cells[i] = [0, 0, new_val, new_val];
                continue;
            }

            for j in 0..3 {
                if old_row[j] == old_row[j+1] {
                    new_row[j] = 0;
                    new_row[j+1] *= 2;
                }
            }
            self.cells[i] = new_row;
        }

        self.mov_all_cells_to_the_side();
    }

    fn mov_all_cells_to_the_side(&mut self) {
        for i in 0..4 {
            let mut row = self.cells[i];
            for j in (0..3).rev() {
                let temp = row[j];
                row[j] = 0;
                let mut index = j;
                while index < 3 {
                    if row[index + 1] != 0 {
                        break;
                    }
                    index += 1;
                }
                row[index] = temp;
            }
            self.cells[i] = row;
        }
    }

    fn rotate(&mut self) {
        let mut cells = self.cells;
        for i in 0..4 {
            for j in i..4 {
                let temp = cells[i][j];
                cells[i][j] = cells[j][i];
                cells[j][i] = temp;
            }
        }
        for i in 0..4 {
            cells[i].reverse()
        }
        self.cells = cells
    }
}

enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Move {
    fn get_number(&self) -> usize {
        match self {
            Move::Left => todo!(),
            Move::Right => 0,
            Move::Up => todo!(),
            Move::Down => 3,
        }
    }
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

        grid.mov();
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

        grid.mov();
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

        grid.mov();
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

        grid.mov();
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

        grid.mov();
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

        grid.mov();
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn rotate_right_works() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 2, 0, 2];
        let row2 = [0, 0, 0, 2];
        let row3 = [0, 4, 0, 2];
        let row4 = [0, 0, 0, 2];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.rotate();
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check2() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 2, 0, 2];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 4, 8];
        let row2 = [0, 0, 0, 4];
        let row3 = [0, 0, 0, 4];
        let row4 = [0, 0, 0, 2];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check3() {
        let row1 = [2, 2, 4, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn sanity_check4() {
        let row1 = [2, 0, 2, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();
        assert_eq!(grid, result_grid)
    }

    fn move_left_works() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [4, 8, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [4, 0, 0, 0];
        let row4 = [2, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_all_to_the_side_works() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 2, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 2, 2];
        let row3 = [0, 0, 2, 2];
        let row4 = [0, 0, 2, 2];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov_all_cells_to_the_side();
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_all_to_the_side_works2() {
        let row1 = [2, 2, 4, 0];
        let row2 = [2, 2, 0, 2];
        let row3 = [4, 0, 2, 2];
        let row4 = [0, 2, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 2, 2, 4];
        let row2 = [0, 2, 2, 2];
        let row3 = [0, 4, 2, 2];
        let row4 = [0, 2, 2, 2];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov_all_cells_to_the_side();
        assert_eq!(grid, result_grid)
    }
}
