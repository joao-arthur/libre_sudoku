use crate::{
    board::{Board, get_cell, get_col, get_row, get_sq, get_sq_idx},
    cell::Cell,
    strategy::{strategy_last_empty_in_group, strategy_only_possibility},
};

fn solve(b: &mut Board) {
    for row_i in 0..9 {
        for col_i in 0..9 {
            if let Some(row_cell) = Cell::try_from_idx(row_i) {
                if let Some(col_cell) = Cell::try_from_idx(col_i) {
                    if get_cell(&b, &row_cell, &col_cell).is_none() {
                        let row = get_row(b, &row_cell);
                        let col = get_col(b, &col_cell);
                        let sq = get_sq(b, &get_sq_idx(&row_cell, &col_cell));
                        let opt_col = strategy_last_empty_in_group(&col);
                        let opt_row = strategy_last_empty_in_group(&row);
                        let opt_sq = strategy_last_empty_in_group(&sq);
                        if let Some(opt_col) = opt_col {
                            b[row_i as usize][col_i as usize] = Some(opt_col);
                        }
                        if let Some(opt_row) = opt_row {
                            b[row_i as usize][col_i as usize] = Some(opt_row);
                        }
                        if let Some(opt_sq) = opt_sq {
                            b[row_i as usize][col_i as usize] = Some(opt_sq);
                        }
                    }
                }
            }
        }
    }
    //strategy_only_possibility()
}

#[cfg(test)]
mod tests {
    use crate::board::{self, to_string};

    use super::*;

    #[test]
    fn test_solve_last_empty_in_group() {
        let mut b = board::from_str([
            "17 549683",
            "6458 3219",
            "389261  5",
            "49632785 ",
            "81345697 ",
            "257198436",
            "964715328",
            "731682594",
            "528934167",
        ])
        .unwrap();
        let solution = board::from_str([
            "172549683",
            "645873219",
            "389261745",
            "496327851",
            "813456972",
            "257198436",
            "964715328",
            "731682594",
            "528934167",
        ])
        .unwrap();
        solve(&mut b);
        assert_eq!(to_string(&b), to_string(&solution));
    }
}
