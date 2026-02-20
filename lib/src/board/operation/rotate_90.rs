use crate::{board::{Board, get_col, get_row}, cell::Cell};

pub fn rotate_90(board: &Board) -> Board {
    unsafe {
        [
            [
                *board.get_unchecked(8).get_unchecked(0),
                *board.get_unchecked(7).get_unchecked(0),
                *board.get_unchecked(6).get_unchecked(0),
                *board.get_unchecked(5).get_unchecked(0),
                *board.get_unchecked(4).get_unchecked(0),
                *board.get_unchecked(3).get_unchecked(0),
                *board.get_unchecked(2).get_unchecked(0),
                *board.get_unchecked(1).get_unchecked(0),
                *board.get_unchecked(0).get_unchecked(0),
            ],
            [
                *board.get_unchecked(8).get_unchecked(1),
                *board.get_unchecked(7).get_unchecked(1),
                *board.get_unchecked(6).get_unchecked(1),
                *board.get_unchecked(5).get_unchecked(1),
                *board.get_unchecked(4).get_unchecked(1),
                *board.get_unchecked(3).get_unchecked(1),
                *board.get_unchecked(2).get_unchecked(1),
                *board.get_unchecked(1).get_unchecked(1),
                *board.get_unchecked(0).get_unchecked(1),
            ],
            [
                *board.get_unchecked(8).get_unchecked(2),
                *board.get_unchecked(7).get_unchecked(2),
                *board.get_unchecked(6).get_unchecked(2),
                *board.get_unchecked(5).get_unchecked(2),
                *board.get_unchecked(4).get_unchecked(2),
                *board.get_unchecked(3).get_unchecked(2),
                *board.get_unchecked(2).get_unchecked(2),
                *board.get_unchecked(1).get_unchecked(2),
                *board.get_unchecked(0).get_unchecked(2),
            ],
                        [
                *board.get_unchecked(8).get_unchecked(3),
                *board.get_unchecked(7).get_unchecked(3),
                *board.get_unchecked(6).get_unchecked(3),
                *board.get_unchecked(5).get_unchecked(3),
                *board.get_unchecked(4).get_unchecked(3),
                *board.get_unchecked(3).get_unchecked(3),
                *board.get_unchecked(2).get_unchecked(3),
                *board.get_unchecked(1).get_unchecked(3),
                *board.get_unchecked(0).get_unchecked(3),
            ],
            [
                *board.get_unchecked(8).get_unchecked(4),
                *board.get_unchecked(7).get_unchecked(4),
                *board.get_unchecked(6).get_unchecked(4),
                *board.get_unchecked(5).get_unchecked(4),
                *board.get_unchecked(4).get_unchecked(4),
                *board.get_unchecked(3).get_unchecked(4),
                *board.get_unchecked(2).get_unchecked(4),
                *board.get_unchecked(1).get_unchecked(4),
                *board.get_unchecked(0).get_unchecked(4),
            ],
            [
                *board.get_unchecked(8).get_unchecked(5),
                *board.get_unchecked(7).get_unchecked(5),
                *board.get_unchecked(6).get_unchecked(5),
                *board.get_unchecked(5).get_unchecked(5),
                *board.get_unchecked(4).get_unchecked(5),
                *board.get_unchecked(3).get_unchecked(5),
                *board.get_unchecked(2).get_unchecked(5),
                *board.get_unchecked(1).get_unchecked(5),
                *board.get_unchecked(0).get_unchecked(5),
            ],
            [
                *board.get_unchecked(8).get_unchecked(6),
                *board.get_unchecked(7).get_unchecked(6),
                *board.get_unchecked(6).get_unchecked(6),
                *board.get_unchecked(5).get_unchecked(6),
                *board.get_unchecked(4).get_unchecked(6),
                *board.get_unchecked(3).get_unchecked(6),
                *board.get_unchecked(2).get_unchecked(6),
                *board.get_unchecked(1).get_unchecked(6),
                *board.get_unchecked(0).get_unchecked(6),
            ],
            [
                *board.get_unchecked(8).get_unchecked(7),
                *board.get_unchecked(7).get_unchecked(7),
                *board.get_unchecked(6).get_unchecked(7),
                *board.get_unchecked(5).get_unchecked(7),
                *board.get_unchecked(4).get_unchecked(7),
                *board.get_unchecked(3).get_unchecked(7),
                *board.get_unchecked(2).get_unchecked(7),
                *board.get_unchecked(1).get_unchecked(7),
                *board.get_unchecked(0).get_unchecked(7),
            ],
            [
                *board.get_unchecked(8).get_unchecked(8),
                *board.get_unchecked(7).get_unchecked(8),
                *board.get_unchecked(6).get_unchecked(8),
                *board.get_unchecked(5).get_unchecked(8),
                *board.get_unchecked(4).get_unchecked(8),
                *board.get_unchecked(3).get_unchecked(8),
                *board.get_unchecked(2).get_unchecked(8),
                *board.get_unchecked(1).get_unchecked(8),
                *board.get_unchecked(0).get_unchecked(8),
            ],
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::board::board_from_str;
    use super::rotate_90;

    #[test]
    fn test_rotate_90() {
        assert_eq!(
            rotate_90(&board_from_str([
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
                "98765432 ",
                "9876543 1",
                "987654 21",
                "98765 321",
                "9876 4321",
                "987 54321",
                "98 654321",
                "9 7654321",
                " 87654321",
            ]),
        );
        assert_eq!(
            rotate_90(&board_from_str([
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
                "987654321",
                "987654321",
                "987654321",
                "987654321",
                "987654321",
                "987654321",
                "987654321",
                "987654321",
                "987654321",
            ]),
        );
    }
}
