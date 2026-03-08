use crate::board::Board;

pub fn permute_group_row_1_3(board: &Board) -> Board {
    unsafe {
        [
            *board.get_unchecked(6),
            *board.get_unchecked(7),
            *board.get_unchecked(8),
            *board.get_unchecked(3),
            *board.get_unchecked(4),
            *board.get_unchecked(5),
            *board.get_unchecked(0),
            *board.get_unchecked(1),
            *board.get_unchecked(2),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::permute_group_row_1_3;
    use crate::board::from_str;

    #[test]
    fn test_permute_group_row_1_3() {
        assert_eq!(
            permute_group_row_1_3(&from_str([
                "111111111",
                "222222222",
                "333333333",
                "444444444",
                "555555555",
                "666666666",
                "777777777",
                "888888888",
                "999999999",
            ])),
            from_str([
                "777777777",
                "888888888",
                "999999999",
                "444444444",
                "555555555",
                "666666666",
                "111111111",
                "222222222",
                "333333333",
            ]),
        );
    }
}
