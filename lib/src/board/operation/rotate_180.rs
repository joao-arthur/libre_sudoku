use crate::board::Board;

pub fn rotate_180(board: &Board) -> Board {
    unsafe {
        [
            [
                *board.get_unchecked(8).get_unchecked(8),
                *board.get_unchecked(8).get_unchecked(7),
                *board.get_unchecked(8).get_unchecked(6),
                *board.get_unchecked(8).get_unchecked(5),
                *board.get_unchecked(8).get_unchecked(4),
                *board.get_unchecked(8).get_unchecked(3),
                *board.get_unchecked(8).get_unchecked(2),
                *board.get_unchecked(8).get_unchecked(1),
                *board.get_unchecked(8).get_unchecked(0),
            ],
            [
                *board.get_unchecked(7).get_unchecked(8),
                *board.get_unchecked(7).get_unchecked(7),
                *board.get_unchecked(7).get_unchecked(6),
                *board.get_unchecked(7).get_unchecked(5),
                *board.get_unchecked(7).get_unchecked(4),
                *board.get_unchecked(7).get_unchecked(3),
                *board.get_unchecked(7).get_unchecked(2),
                *board.get_unchecked(7).get_unchecked(1),
                *board.get_unchecked(7).get_unchecked(0),
            ],
            [
                *board.get_unchecked(6).get_unchecked(8),
                *board.get_unchecked(6).get_unchecked(7),
                *board.get_unchecked(6).get_unchecked(6),
                *board.get_unchecked(6).get_unchecked(5),
                *board.get_unchecked(6).get_unchecked(4),
                *board.get_unchecked(6).get_unchecked(3),
                *board.get_unchecked(6).get_unchecked(2),
                *board.get_unchecked(6).get_unchecked(1),
                *board.get_unchecked(6).get_unchecked(0),
            ],
            [
                *board.get_unchecked(5).get_unchecked(8),
                *board.get_unchecked(5).get_unchecked(7),
                *board.get_unchecked(5).get_unchecked(6),
                *board.get_unchecked(5).get_unchecked(5),
                *board.get_unchecked(5).get_unchecked(4),
                *board.get_unchecked(5).get_unchecked(3),
                *board.get_unchecked(5).get_unchecked(2),
                *board.get_unchecked(5).get_unchecked(1),
                *board.get_unchecked(5).get_unchecked(0),
            ],
            [
                *board.get_unchecked(4).get_unchecked(8),
                *board.get_unchecked(4).get_unchecked(7),
                *board.get_unchecked(4).get_unchecked(6),
                *board.get_unchecked(4).get_unchecked(5),
                *board.get_unchecked(4).get_unchecked(4),
                *board.get_unchecked(4).get_unchecked(3),
                *board.get_unchecked(4).get_unchecked(2),
                *board.get_unchecked(4).get_unchecked(1),
                *board.get_unchecked(4).get_unchecked(0),
            ],
            [
                *board.get_unchecked(3).get_unchecked(8),
                *board.get_unchecked(3).get_unchecked(7),
                *board.get_unchecked(3).get_unchecked(6),
                *board.get_unchecked(3).get_unchecked(5),
                *board.get_unchecked(3).get_unchecked(4),
                *board.get_unchecked(3).get_unchecked(3),
                *board.get_unchecked(3).get_unchecked(2),
                *board.get_unchecked(3).get_unchecked(1),
                *board.get_unchecked(3).get_unchecked(0),
            ],
            [
                *board.get_unchecked(2).get_unchecked(8),
                *board.get_unchecked(2).get_unchecked(7),
                *board.get_unchecked(2).get_unchecked(6),
                *board.get_unchecked(2).get_unchecked(5),
                *board.get_unchecked(2).get_unchecked(4),
                *board.get_unchecked(2).get_unchecked(3),
                *board.get_unchecked(2).get_unchecked(2),
                *board.get_unchecked(2).get_unchecked(1),
                *board.get_unchecked(2).get_unchecked(0),
            ],
            [
                *board.get_unchecked(1).get_unchecked(8),
                *board.get_unchecked(1).get_unchecked(7),
                *board.get_unchecked(1).get_unchecked(6),
                *board.get_unchecked(1).get_unchecked(5),
                *board.get_unchecked(1).get_unchecked(4),
                *board.get_unchecked(1).get_unchecked(3),
                *board.get_unchecked(1).get_unchecked(2),
                *board.get_unchecked(1).get_unchecked(1),
                *board.get_unchecked(1).get_unchecked(0),
            ],
            [
                *board.get_unchecked(0).get_unchecked(8),
                *board.get_unchecked(0).get_unchecked(7),
                *board.get_unchecked(0).get_unchecked(6),
                *board.get_unchecked(0).get_unchecked(5),
                *board.get_unchecked(0).get_unchecked(4),
                *board.get_unchecked(0).get_unchecked(3),
                *board.get_unchecked(0).get_unchecked(2),
                *board.get_unchecked(0).get_unchecked(1),
                *board.get_unchecked(0).get_unchecked(0),
            ],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::rotate_180;
    use crate::board::board_from_str;

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
                " 99999999",
                "8 8888888",
                "77 777777",
                "666 66666",
                "5555 5555",
                "44444 444",
                "333333 33",
                "2222222 2",
                "11111111 ",
            ]),
        );
        assert_eq!(
            rotate_180(&board_from_str([
                "123456789",
                "123456789",
                "123456789",
                "123456789",
                "123456789",
                "123456789",
                "123456789",
                "123456789",
                "123456789",
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
