use crate::{board::Board, cell::Cell};

const CELLS: [Cell; 9] =
    [Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6, Cell::_7, Cell::_8, Cell::_9];

fn brute_force_solve(board: &Board) -> Option<Board> {
    let mut result = board.clone();
    let mut cells_to_guess: Vec<(usize, usize)> = Vec::new();
    for row in 0..9 {
        for col in 0..9 {
            unsafe {
                if result.get_unchecked(row).get_unchecked(col).is_none() {
                    cells_to_guess.push((row, col));
                }
            }
        }
    }
    let mut curr_guess_i = 0;
    'outer: loop {
        let (row_i, col_i) = cells_to_guess[curr_guess_i];
        let mut cell_i = 0;
        let curr_cell = result[row_i][col_i];
        if curr_cell == Some(Cell::_9) {
            result[row_i][col_i] = None;
            if curr_guess_i == 0 {
                break 'outer;
            }
            curr_guess_i -= 1;
            continue;
        }
        if let Some(curr_cell) = curr_cell {
            cell_i = curr_cell.to_usize();
        }
        loop {
            if cell_i > 8 {
                break;
            }
            let cell = CELLS[cell_i];
            if can_place(&result, cell, row_i, col_i) {
                result[row_i][col_i] = Some(cell);
                break;
            }
            cell_i += 1;
        }
        if result[row_i][col_i].is_none() {
            if curr_guess_i == 0 {
                break 'outer;
            }
            curr_guess_i -= 1;
        } else {
            if curr_guess_i == cells_to_guess.len() - 1 {
                break 'outer;
            }
            curr_guess_i += 1;
        }
    }
    Some(result)
}

fn can_place(board: &Board, guess: Cell, row: usize, col: usize) -> bool {
    let some_guess = Some(guess);
    for i in 0..9 {
        unsafe {
            if board.get_unchecked(row).get_unchecked(i) == &some_guess {
                return false;
            }
        }
    }
    for i in 0..9 {
        unsafe {
            if board.get_unchecked(i).get_unchecked(col) == &some_guess {
                return false;
            }
        }
    }
    let sq_row = (row / 3) * 3;
    let sq_col = (col / 3) * 3;
    for row in 0..3 {
        for col in 0..3 {
            unsafe {
                if board.get_unchecked(sq_row + row).get_unchecked(sq_col + col) == &some_guess {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::brute_force_solve;
    use crate::board::{from_str, to_string};

    #[test]
    fn test_solve_brute_force() {
        //        let board = from_str([
        //            " 8   5179",
        //            "   2 6 84",
        //            "9 3   6  ",
        //            "27  8 5 3",
        //            "4   5 812",
        //            "  8 42  7",
        //            "8    3  1",
        //            "354 1  9 ",
        //            " 96 247  ",
        //        ]);
        let board = from_str([
            " 8   5179",
            "   2 6 84",
            "9 3   6  ",
            "27  8 5 3",
            "4   5 812",
            "538142967",
            "827963451",
            "354718296",
            "196524738",
        ]);
        let solution = from_str([
            "682435179",
            "715296384",
            "943871625",
            "271689543",
            "469357812",
            "538142967",
            "827963451",
            "354718296",
            "196524738",
        ]);
        assert_eq!(to_string(&brute_force_solve(&board).unwrap()), to_string(&solution));
    }
}
