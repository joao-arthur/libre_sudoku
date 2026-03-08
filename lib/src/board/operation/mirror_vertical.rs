use crate::board::Board;

pub fn mirror_vertical(board: &Board) -> Board {
    unsafe {
        [
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
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::mirror_vertical;
    use crate::board::from_str;

    #[test]
    fn test_mirror_vertical() {
        assert_eq!(
            mirror_vertical(&from_str([
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
            from_str([
                "11111111 ",
                "2222222 2",
                "333333 33",
                "44444 444",
                "5555 5555",
                "666 66666",
                "77 777777",
                "8 8888888",
                " 99999999",
            ]),
        );
        assert_eq!(
            mirror_vertical(&from_str([
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
            from_str([
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
