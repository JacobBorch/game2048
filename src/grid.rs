use std::error::Error;

use rand::{Rng};

#[derive(PartialEq, Debug)]
pub struct Grid {
    // 4x4 grid
    cells: [[u64; 4]; 4],
}

enum GameStatus {
    Done
}

enum GameError {

}

impl Grid {
    fn new(cells: [[u64; 4]; 4]) -> Self {
        Self { cells }
    }

    fn attempt(&mut self, mov: Move) -> Result<GameStatus, GameError> {

        self.make_move(mov);
        self.insert_random_cell();
        Ok(GameStatus::Done)
    }

    fn insert_random_cell(&mut self) {
        if self.is_board_full() {
            return;
        }
        let random_cell: u64 = if rand::random() {2} else {4};
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..4usize);
            let y = rng.gen_range(0..4usize);
            if self.cells[x][y] == 0 {
                self.cells[x][y] = random_cell;
                break;
            }
        }
    }

    fn is_board_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|val| *val != 0))
    }

    fn make_move(&mut self, mov: Move) {
        let rotation = mov.get_number();
        self.handle_move(rotation)
    }

    fn handle_move(&mut self, rotation: usize) {
        self.rotate_times(rotation);
        self.mov();
        self.rotate_times(4-rotation)
    }

    fn rotate_times(&mut self, n: usize) {
        for _i in 0..n {
            self.rotate()
        }
    }

    fn mov(&mut self) {
        // Implementation of Going right.
        self.mov_all_cells_to_the_side();

        for i in 0..4 {
            let old_row = self.cells[i];
            let mut new_row = old_row;

            //SPECIAL CASE: [x, x, x, x] => [0, 0, 2x, 2x]
            let val = old_row[0];
            if old_row.iter().all(|x| *x == val) {
                let new_val = 2 * val;
                self.cells[i] = [0, 0, new_val, new_val];
                continue;
            }

            // Normal case
            for j in (1..=3).rev() {
                if new_row[j] == old_row[j-1] {
                    new_row[j] *= 2;
                    new_row[j-1] = 0;
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
            Move::Right => 0,
            Move::Up => 1,
            Move::Left => 2,
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
    fn rotate_twice_works() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 0];
        let row2 = [0, 4, 0, 2];
        let row3 = [0, 0, 0, 0];
        let row4 = [2, 2, 2, 2];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.rotate_times(2);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn lol() {
        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [2, 2, 2, 2];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 4, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.rotate_times(2);
        grid.rotate_times(2);
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

    #[test]
    fn something() {
        let row1 = [0, 0, 2, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 4, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 8, 4];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();
        assert_eq!(grid, result_grid)
    }

    #[test]
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

        grid.make_move(Move::Left);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_up_works() {
        let row1 = [2, 2, 4, 4];
        let row2 = [0, 0, 0, 0];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [4, 4, 4, 4];
        let row2 = [0, 0, 2, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.make_move(Move::Up);
        assert_eq!(grid, result_grid)
    }

    #[test]
    fn move_down_works() {
        let row1 = [2, 0, 0, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [2, 0, 2, 0];
        let row4 = [0, 2, 2, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 0];
        let row2 = [0, 0, 0, 0];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 4, 4, 2];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.make_move(Move::Down);
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

    #[test]
    fn random_cell_is_inserted_after_attempt() {
        let row1 = [0, 0, 2, 0];
        let row2 = [0, 2, 0, 2];
        let row3 = [0, 0, 0, 0];
        let row4 = [4, 4, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 0, 0, 2];
        let row2 = [0, 0, 0, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 8, 4];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.attempt(Move::Right);
        assert_ne!(grid, result_grid);
    }

    #[test]
    fn board_is_full_works() {
        let row1 = [2, 2, 4, 2];
        let row2 = [2, 2, 2, 2];
        let row3 = [4, 2, 2, 2];
        let row4 = [2, 2, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        assert_eq!(grid.is_board_full(), true);

        let row1 = [2, 2, 4, 2];
        let row2 = [2, 2, 0, 2];
        let row3 = [4, 2, 2, 2];
        let row4 = [2, 2, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        assert_eq!(grid.is_board_full(), false)
    }

    #[test]
    fn insert_random_cell_works() {
        let row1 = [2, 2, 4, 2];
        let row2 = [2, 0, 2, 2];
        let row3 = [4, 2, 2, 2];
        let row4 = [2, 2, 2, 2];
        let mut grid = Grid::new([row1, row2, row3, row4]);
        grid.insert_random_cell();

        let cell = grid.cells[1][1];
        assert!(cell == 2 || cell == 4)
    }

    #[test]
    fn nothing_moves_when_nothing_should_move() {
        let row1 = [2, 4, 8, 16];
        let row2 = [2, 4, 8, 16];
        let row3 = [2, 4, 8, 16];
        let row4 = [2, 4, 8, 16];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [2, 4, 8, 16];
        let row2 = [2, 4, 8, 16];
        let row3 = [2, 4, 8, 16];
        let row4 = [2, 4, 8, 16];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();

        assert_eq!(grid, result_grid)
    }

    #[test]
    fn bad_feeling_bout_this() {
        let row1 = [2, 2, 2, 4];
        let row2 = [2, 4, 4, 4];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let mut grid = Grid::new([row1, row2, row3, row4]);

        let row1 = [0, 2, 4, 4];
        let row2 = [0, 2, 4, 8];
        let row3 = [0, 0, 0, 0];
        let row4 = [0, 0, 0, 0];
        let result_grid = Grid::new([row1, row2, row3, row4]);

        grid.mov();
        assert_eq!(grid, result_grid);
    }
}
