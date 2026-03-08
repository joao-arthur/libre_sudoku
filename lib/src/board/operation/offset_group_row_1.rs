use crate::board::Board;

pub fn offset_group_row_1(board: &Board) -> Board {
    unsafe {
        [
            *board.get_unchecked(6),
            *board.get_unchecked(7),
            *board.get_unchecked(8),
            *board.get_unchecked(0),
            *board.get_unchecked(1),
            *board.get_unchecked(2),
            *board.get_unchecked(3),
            *board.get_unchecked(4),
            *board.get_unchecked(5),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::offset_group_row_1;
    use crate::board::from_str;

    #[test]
    fn test_offset_group_row_1() {
        assert_eq!(
            offset_group_row_1(&from_str([
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
                "111111111",
                "222222222",
                "333333333",
                "444444444",
                "555555555",
                "666666666",
            ]),
        );
    }
}
