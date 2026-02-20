use crate::{
    board::{self, Board},
    cell::Cell,
    strategy::strategy_last_empty_in_group,
};

fn solve(b: &mut Board) {
    for row_i in 0..9 {
        for col_i in 0..9 {
            if let Some(row_cell) = Cell::try_from_usize(row_i)
                && let Some(col_cell) = Cell::try_from_usize(col_i)
                && board::get_cell(b, &row_cell, &col_cell).is_none()
            {
                let row = board::get_row(b, &row_cell);
                let col = board::get_col(b, &col_cell);
                let sq = board::get_sq(b, &board::get_sq_idx(&row_cell, &col_cell));
                let opt_col = strategy_last_empty_in_group(&col);
                let opt_row = strategy_last_empty_in_group(&row);
                let opt_sq = strategy_last_empty_in_group(&sq);
                if let Some(opt_col) = opt_col {
                    b[row_i][col_i] = Some(opt_col);
                }
                if let Some(opt_row) = opt_row {
                    b[row_i][col_i] = Some(opt_row);
                }
                if let Some(opt_sq) = opt_sq {
                    b[row_i][col_i] = Some(opt_sq);
                }
            }
        }
    }
    //strategy_only_possibility()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::{board_from_str, board_to_string};

    #[test]
    fn test_solve_last_empty_in_group() {
        let mut b = board_from_str([
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
        let solution = board_from_str([
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
        solve(&mut b);
        assert_eq!(board_to_string(&b), board_to_string(&solution));
    }
}
