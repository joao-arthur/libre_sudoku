use crate::{board::Board, cell::Cell};

pub fn offset(board: &Board, by: &Cell) -> Board {
    unsafe {
        [
            [
                (*board.get_unchecked(0).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(0).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(1).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(1).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(2).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(2).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(3).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(3).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(4).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(4).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(5).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(5).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(6).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(6).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(7).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(7).get_unchecked(8)).map(|cell| cell + *by),
            ],
            [
                (*board.get_unchecked(8).get_unchecked(0)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(1)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(2)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(3)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(4)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(5)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(6)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(7)).map(|cell| cell + *by),
                (*board.get_unchecked(8).get_unchecked(8)).map(|cell| cell + *by),
            ],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::offset;
    use crate::{board::from_str, cell::Cell};

    #[test]
    fn test_offset() {
        assert_eq!(
            offset(
                &from_str([
                    " 11111111",
                    "2 2222222",
                    "33 333333",
                    "444 44444",
                    "5555 5555",
                    "66666 666",
                    "777777 77",
                    "8888888 8",
                    "99999999 ",
                ]),
                &Cell::_1
            ),
            from_str([
                " 22222222",
                "3 3333333",
                "44 444444",
                "555 55555",
                "6666 6666",
                "77777 777",
                "888888 88",
                "9999999 9",
                "11111111 ",
            ]),
        );
        assert_eq!(
            offset(
                &from_str([
                    " 11111111",
                    "2 2222222",
                    "33 333333",
                    "444 44444",
                    "5555 5555",
                    "66666 666",
                    "777777 77",
                    "8888888 8",
                    "99999999 ",
                ]),
                &Cell::_5
            ),
            from_str([
                " 66666666",
                "7 7777777",
                "88 888888",
                "999 99999",
                "1111 1111",
                "22222 222",
                "333333 33",
                "4444444 4",
                "55555555 ",
            ]),
        );
        assert_eq!(
            offset(
                &from_str([
                    " 11111111",
                    "2 2222222",
                    "33 333333",
                    "444 44444",
                    "5555 5555",
                    "66666 666",
                    "777777 77",
                    "8888888 8",
                    "99999999 ",
                ]),
                &Cell::_9
            ),
            from_str([
                " 11111111",
                "2 2222222",
                "33 333333",
                "444 44444",
                "5555 5555",
                "66666 666",
                "777777 77",
                "8888888 8",
                "99999999 ",
            ]),
        );
    }
}
