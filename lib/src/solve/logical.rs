use crate::{
    board::{Board, get_cell, get_col, get_row, get_sq, get_sq_idx},
    cell::Cell,
    strategy::strategy_last_empty_in_group,
};

fn logical_solve(board: &mut Board) {
    for row_i in 0..9 {
        for col_i in 0..9 {
            let row_cell = Cell::from_usize(row_i);
            let col_cell = Cell::from_usize(col_i);
            if get_cell(board, &row_cell, &col_cell).is_none() {
                let row = get_row(board, &row_cell);
                let col = get_col(board, &col_cell);
                let sq = get_sq(board, &get_sq_idx(&row_cell, &col_cell));
                if let Some(opt_col) = strategy_last_empty_in_group(&col) {
                    board[row_i][col_i] = Some(opt_col);
                }
                if let Some(opt_row) = strategy_last_empty_in_group(&row) {
                    board[row_i][col_i] = Some(opt_row);
                }
                if let Some(opt_sq) = strategy_last_empty_in_group(&sq) {
                    board[row_i][col_i] = Some(opt_sq);
                }
            }
        }
    }
    //strategy_only_possibility()
}

#[cfg(test)]
mod tests {
    use super::logical_solve;
    use crate::board::{from_str, to_string};

    #[test]
    fn test_solve_last_empty_in_group() {
        let mut b = from_str([
            "17 549683",
            "6458 3219",
            "389261  5",
            "49632785 ",
            "81345697 ",
            "257198436",
            "964715328",
            "731682594",
            "528934167",
        ]);
        let solution = from_str([
            "172549683",
            "645873219",
            "389261745",
            "496327851",
            "813456972",
            "257198436",
            "964715328",
            "731682594",
            "528934167",
        ]);
        logical_solve(&mut b);
        assert_eq!(to_string(&b), to_string(&solution));
    }
}
