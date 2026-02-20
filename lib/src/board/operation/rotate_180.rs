use crate::{board::{Board, get_col, get_row}, cell::Cell};

pub fn rotate_180(board: &Board) -> Board {
    unsafe {
        [
            [
                *board.get_unchecked(8).get_unchecked(0),
                *board.get_unchecked(8).get_unchecked(1),
                *board.get_unchecked(8).get_unchecked(2),
                *board.get_unchecked(8).get_unchecked(3),
                *board.get_unchecked(8).get_unchecked(4),
                *board.get_unchecked(8).get_unchecked(5),
                *board.get_unchecked(8).get_unchecked(6),
                *board.get_unchecked(8).get_unchecked(7),
                *board.get_unchecked(8).get_unchecked(8),
            ],
            [
                *board.get_unchecked(7).get_unchecked(0),
                *board.get_unchecked(7).get_unchecked(1),
                *board.get_unchecked(7).get_unchecked(2),
                *board.get_unchecked(7).get_unchecked(3),
                *board.get_unchecked(7).get_unchecked(4),
                *board.get_unchecked(7).get_unchecked(5),
                *board.get_unchecked(7).get_unchecked(6),
                *board.get_unchecked(7).get_unchecked(7),
                *board.get_unchecked(7).get_unchecked(8),
            ],
            [
                *board.get_unchecked(6).get_unchecked(0),
                *board.get_unchecked(6).get_unchecked(1),
                *board.get_unchecked(6).get_unchecked(2),
                *board.get_unchecked(6).get_unchecked(3),
                *board.get_unchecked(6).get_unchecked(4),
                *board.get_unchecked(6).get_unchecked(5),
                *board.get_unchecked(6).get_unchecked(6),
                *board.get_unchecked(6).get_unchecked(7),
                *board.get_unchecked(6).get_unchecked(8),
            ],
                        [
                *board.get_unchecked(5).get_unchecked(0),
                *board.get_unchecked(5).get_unchecked(1),
                *board.get_unchecked(5).get_unchecked(2),
                *board.get_unchecked(5).get_unchecked(3),
                *board.get_unchecked(5).get_unchecked(4),
                *board.get_unchecked(5).get_unchecked(5),
                *board.get_unchecked(5).get_unchecked(6),
                *board.get_unchecked(5).get_unchecked(7),
                *board.get_unchecked(5).get_unchecked(8),
            ],
            [
                *board.get_unchecked(4).get_unchecked(0),
                *board.get_unchecked(4).get_unchecked(1),
                *board.get_unchecked(4).get_unchecked(2),
                *board.get_unchecked(4).get_unchecked(3),
                *board.get_unchecked(4).get_unchecked(4),
                *board.get_unchecked(4).get_unchecked(5),
                *board.get_unchecked(4).get_unchecked(6),
                *board.get_unchecked(4).get_unchecked(7),
                *board.get_unchecked(4).get_unchecked(8),
            ],
            [
                *board.get_unchecked(3).get_unchecked(0),
                *board.get_unchecked(3).get_unchecked(1),
                *board.get_unchecked(3).get_unchecked(2),
                *board.get_unchecked(3).get_unchecked(3),
                *board.get_unchecked(3).get_unchecked(4),
                *board.get_unchecked(3).get_unchecked(5),
                *board.get_unchecked(3).get_unchecked(6),
                *board.get_unchecked(3).get_unchecked(7),
                *board.get_unchecked(3).get_unchecked(8),
            ],
            [
                *board.get_unchecked(2).get_unchecked(0),
                *board.get_unchecked(2).get_unchecked(1),
                *board.get_unchecked(2).get_unchecked(2),
                *board.get_unchecked(2).get_unchecked(3),
                *board.get_unchecked(2).get_unchecked(4),
                *board.get_unchecked(2).get_unchecked(5),
                *board.get_unchecked(2).get_unchecked(6),
                *board.get_unchecked(2).get_unchecked(7),
                *board.get_unchecked(2).get_unchecked(8),
            ],
            [
                *board.get_unchecked(1).get_unchecked(0),
                *board.get_unchecked(1).get_unchecked(1),
                *board.get_unchecked(1).get_unchecked(2),
                *board.get_unchecked(1).get_unchecked(3),
                *board.get_unchecked(1).get_unchecked(4),
                *board.get_unchecked(1).get_unchecked(5),
                *board.get_unchecked(1).get_unchecked(6),
                *board.get_unchecked(1).get_unchecked(7),
                *board.get_unchecked(1).get_unchecked(8),
            ],
            [
                *board.get_unchecked(0).get_unchecked(0),
                *board.get_unchecked(0).get_unchecked(1),
                *board.get_unchecked(0).get_unchecked(2),
                *board.get_unchecked(0).get_unchecked(3),
                *board.get_unchecked(0).get_unchecked(4),
                *board.get_unchecked(0).get_unchecked(5),
                *board.get_unchecked(0).get_unchecked(6),
                *board.get_unchecked(0).get_unchecked(7),
                *board.get_unchecked(0).get_unchecked(8),
            ],
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::board::board_from_str;
    use super::rotate_180;

    #[test]
    fn test_rotate_90() {
        assert_eq!(
            rotate_180(&board_from_str([
                " 11111111",
                "2 2222222",
                "33 333333",
                "444 44444",
                "5555 5555",
                "66666 666",
                "777777 77",
                "8888888 8",
                "99999999 ",
            ])),
            board_from_str([
                "99999999 ",
                "8888888 8",
                "777777 77",
                "66666 666",
                "5555 5555",
                "444 44444",
                "33 333333",
                "2 2222222",
                " 11111111",
            ]),
        );
        assert_eq!(
            rotate_180(&board_from_str([
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
            board_from_str([
                "999999999",
                "888888888",
                "777777777",
                "666666666",
                "555555555",
                "444444444",
                "333333333",
                "222222222",
                "111111111",
            ]),
        );
    }
}
