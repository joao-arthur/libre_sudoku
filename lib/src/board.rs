use crate::{
    cell::Cell,
    group::{Group, group_from_str},
};
use std::fmt;

pub type Board = [Group; 9];

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Must match the pattern [0-9 ]")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The length of every line must be 9")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn try_from_str(rows: [&str; 9]) -> Result<Board, FromStringErr> {
    if !rows
        .join("")
        .replace("0", "")
        .replace("1", "")
        .replace("2", "")
        .replace("3", "")
        .replace("4", "")
        .replace("5", "")
        .replace("6", "")
        .replace("7", "")
        .replace("8", "")
        .replace("9", "")
        .replace(" ", "")
        .is_empty()
    {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    for line in rows {
        if line.chars().count() != 9 {
            return Err(FromStringErr::InvalidLength(InvalidLengthErr));
        }
    }
    unsafe {
        Ok([
            group_from_str(rows.get_unchecked(0)),
            group_from_str(rows.get_unchecked(1)),
            group_from_str(rows.get_unchecked(2)),
            group_from_str(rows.get_unchecked(3)),
            group_from_str(rows.get_unchecked(4)),
            group_from_str(rows.get_unchecked(5)),
            group_from_str(rows.get_unchecked(6)),
            group_from_str(rows.get_unchecked(7)),
            group_from_str(rows.get_unchecked(8)),
        ])
    }
}

pub fn from_str(rows: [&str; 9]) -> Board {
    try_from_str(rows).unwrap()
}

pub fn get_col(board: &Board, cell: &Cell) -> Group {
    let i = cell.to_usize();
    unsafe {
        [
            *board.get_unchecked(0).get_unchecked(i),
            *board.get_unchecked(1).get_unchecked(i),
            *board.get_unchecked(2).get_unchecked(i),
            *board.get_unchecked(3).get_unchecked(i),
            *board.get_unchecked(4).get_unchecked(i),
            *board.get_unchecked(5).get_unchecked(i),
            *board.get_unchecked(6).get_unchecked(i),
            *board.get_unchecked(7).get_unchecked(i),
            *board.get_unchecked(8).get_unchecked(i),
        ]
    }
}

pub fn get_row(board: &Board, cell: &Cell) -> Group {
    unsafe { *board.get_unchecked(cell.to_usize()) }
}

pub fn get_sq(board: &Board, cell: &Cell) -> Group {
    let i = cell.to_usize();
    let row_i = i / 3;
    let col_i = i - row_i * 3;
    unsafe {
        [
            *board.get_unchecked(row_i * 3).get_unchecked(col_i * 3),
            *board.get_unchecked(row_i * 3).get_unchecked(col_i * 3 + 1),
            *board.get_unchecked(row_i * 3).get_unchecked(col_i * 3 + 2),
            *board.get_unchecked(row_i * 3 + 1).get_unchecked(col_i * 3),
            *board.get_unchecked(row_i * 3 + 1).get_unchecked(col_i * 3 + 1),
            *board.get_unchecked(row_i * 3 + 1).get_unchecked(col_i * 3 + 2),
            *board.get_unchecked(row_i * 3 + 2).get_unchecked(col_i * 3),
            *board.get_unchecked(row_i * 3 + 2).get_unchecked(col_i * 3 + 1),
            *board.get_unchecked(row_i * 3 + 2).get_unchecked(col_i * 3 + 2),
        ]
    }
}

pub fn get_cell(board: &Board, row: &Cell, col: &Cell) -> Option<Cell> {
    unsafe { *board.get_unchecked(row.to_usize()).get_unchecked(col.to_usize()) }
}

pub fn get_sq_idx(row: &Cell, col: &Cell) -> Cell {
    let row_i = row.to_u8() / 3;
    let col_i = col.to_u8() / 3;
    Cell::from_u8(row_i * 3 + col_i)
}

pub fn to_string(board: &Board) -> String {
    let mut res = String::from("");
    for row in board {
        for col in row {
            match col {
                Some(val) => res.push_str(val.to_str()),
                None => res.push(' '),
            }
        }
        res.push('\n')
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{
        from_str, get_cell, get_col, get_row, get_sq, get_sq_idx, to_string, try_from_str,
    };
    use crate::{cell::Cell, group::group_from_str};

    #[test]
    fn test_try_from_str() {
        assert_eq!(
            try_from_str([
                " 23456789",
                "1 3456789",
                "12 456789",
                "123 56789",
                "1234 6789",
                "12345 789",
                "123456 89",
                "1234567 9",
                "12345678 ",
            ]),
            Ok([
                group_from_str(" 23456789"),
                group_from_str("1 3456789"),
                group_from_str("12 456789"),
                group_from_str("123 56789"),
                group_from_str("1234 6789"),
                group_from_str("12345 789"),
                group_from_str("123456 89"),
                group_from_str("1234567 9"),
                group_from_str("12345678 "),
            ])
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            from_str([
                " 23456789",
                "1 3456789",
                "12 456789",
                "123 56789",
                "1234 6789",
                "12345 789",
                "123456 89",
                "1234567 9",
                "12345678 ",
            ]),
            [
                group_from_str(" 23456789"),
                group_from_str("1 3456789"),
                group_from_str("12 456789"),
                group_from_str("123 56789"),
                group_from_str("1234 6789"),
                group_from_str("12345 789"),
                group_from_str("123456 89"),
                group_from_str("1234567 9"),
                group_from_str("12345678 "),
            ]
        );
    }

    #[test]
    fn test_get_col() {
        let board = from_str([
            " 23456789",
            "1 3456789",
            "12 456789",
            "123 56789",
            "1234 6789",
            "12345 789",
            "123456 89",
            "1234567 9",
            "12345678 ",
        ]);
        assert_eq!(get_col(&board, &Cell::_1), group_from_str(" 11111111"));
        assert_eq!(get_col(&board, &Cell::_2), group_from_str("2 2222222"));
        assert_eq!(get_col(&board, &Cell::_3), group_from_str("33 333333"));
        assert_eq!(get_col(&board, &Cell::_4), group_from_str("444 44444"));
        assert_eq!(get_col(&board, &Cell::_5), group_from_str("5555 5555"));
        assert_eq!(get_col(&board, &Cell::_6), group_from_str("66666 666"));
        assert_eq!(get_col(&board, &Cell::_7), group_from_str("777777 77"));
        assert_eq!(get_col(&board, &Cell::_8), group_from_str("8888888 8"));
        assert_eq!(get_col(&board, &Cell::_9), group_from_str("99999999 "));
    }

    #[test]
    fn test_get_row() {
        let board = from_str([
            " 11111111",
            "2 2222222",
            "33 333333",
            "444 44444",
            "5555 5555",
            "66666 666",
            "777777 77",
            "8888888 8",
            "99999999 ",
        ]);
        assert_eq!(get_row(&board, &Cell::_1), group_from_str(" 11111111"));
        assert_eq!(get_row(&board, &Cell::_2), group_from_str("2 2222222"));
        assert_eq!(get_row(&board, &Cell::_3), group_from_str("33 333333"));
        assert_eq!(get_row(&board, &Cell::_4), group_from_str("444 44444"));
        assert_eq!(get_row(&board, &Cell::_5), group_from_str("5555 5555"));
        assert_eq!(get_row(&board, &Cell::_6), group_from_str("66666 666"));
        assert_eq!(get_row(&board, &Cell::_7), group_from_str("777777 77"));
        assert_eq!(get_row(&board, &Cell::_8), group_from_str("8888888 8"));
        assert_eq!(get_row(&board, &Cell::_9), group_from_str("99999999 "));
    }

    #[test]
    fn test_get_sq() {
        let board = from_str([
            " 112 233 ",
            "111222333",
            "111222333",
            "444555666",
            " 445 566 ",
            "444555666",
            "777888999",
            "777888999",
            " 778 899 ",
        ]);
        assert_eq!(get_sq(&board, &Cell::_1), group_from_str(" 11111111"));
        assert_eq!(get_sq(&board, &Cell::_2), group_from_str("2 2222222"));
        assert_eq!(get_sq(&board, &Cell::_3), group_from_str("33 333333"));
        assert_eq!(get_sq(&board, &Cell::_4), group_from_str("444 44444"));
        assert_eq!(get_sq(&board, &Cell::_5), group_from_str("5555 5555"));
        assert_eq!(get_sq(&board, &Cell::_6), group_from_str("66666 666"));
        assert_eq!(get_sq(&board, &Cell::_7), group_from_str("777777 77"));
        assert_eq!(get_sq(&board, &Cell::_8), group_from_str("8888888 8"));
        assert_eq!(get_sq(&board, &Cell::_9), group_from_str("99999999 "));
    }

    #[test]
    fn test_get_cell() {
        let board = from_str([
            " 23456789",
            "1 3456789",
            "12 456789",
            "123 56789",
            "1234 6789",
            "12345 789",
            "123456 89",
            "1234567 9",
            "12345678 ",
        ]);
        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_1), None);
        assert_eq!(get_cell(&board, &Cell::_2, &Cell::_2), None);
        assert_eq!(get_cell(&board, &Cell::_3, &Cell::_3), None);

        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_2), Some(Cell::_2));
        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_3), Some(Cell::_3));
        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_4), Some(Cell::_4));

        assert_eq!(get_cell(&board, &Cell::_9, &Cell::_6), Some(Cell::_6));
        assert_eq!(get_cell(&board, &Cell::_9, &Cell::_7), Some(Cell::_7));
        assert_eq!(get_cell(&board, &Cell::_9, &Cell::_8), Some(Cell::_8));
    }

    #[test]
    fn test_get_sq_idx() {
        assert_eq!(get_sq_idx(&Cell::_1, &Cell::_1), Cell::_1);
        assert_eq!(get_sq_idx(&Cell::_1, &Cell::_2), Cell::_1);
        assert_eq!(get_sq_idx(&Cell::_1, &Cell::_3), Cell::_1);

        assert_eq!(get_sq_idx(&Cell::_4, &Cell::_1), Cell::_4);
        assert_eq!(get_sq_idx(&Cell::_4, &Cell::_2), Cell::_4);
        assert_eq!(get_sq_idx(&Cell::_4, &Cell::_3), Cell::_4);

        assert_eq!(get_sq_idx(&Cell::_7, &Cell::_1), Cell::_7);
        assert_eq!(get_sq_idx(&Cell::_7, &Cell::_2), Cell::_7);
        assert_eq!(get_sq_idx(&Cell::_7, &Cell::_3), Cell::_7);

        assert_eq!(get_sq_idx(&Cell::_3, &Cell::_7), Cell::_3);
        assert_eq!(get_sq_idx(&Cell::_3, &Cell::_8), Cell::_3);
        assert_eq!(get_sq_idx(&Cell::_3, &Cell::_9), Cell::_3);

        assert_eq!(get_sq_idx(&Cell::_6, &Cell::_7), Cell::_6);
        assert_eq!(get_sq_idx(&Cell::_6, &Cell::_8), Cell::_6);
        assert_eq!(get_sq_idx(&Cell::_6, &Cell::_9), Cell::_6);

        assert_eq!(get_sq_idx(&Cell::_9, &Cell::_7), Cell::_9);
        assert_eq!(get_sq_idx(&Cell::_9, &Cell::_8), Cell::_9);
        assert_eq!(get_sq_idx(&Cell::_9, &Cell::_9), Cell::_9);

        assert_eq!(get_sq_idx(&Cell::_4, &Cell::_4), Cell::_5);
        assert_eq!(get_sq_idx(&Cell::_5, &Cell::_5), Cell::_5);
        assert_eq!(get_sq_idx(&Cell::_6, &Cell::_6), Cell::_5);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            to_string(
                &try_from_str([
                    " 23456789",
                    "1 3456789",
                    "12 456789",
                    "123 56789",
                    "1234 6789",
                    "12345 789",
                    "123456 89",
                    "1234567 9",
                    "12345678 ",
                ])
                .unwrap()
            ),
            String::from(" 23456789\n")
                + "1 3456789\n"
                + "12 456789\n"
                + "123 56789\n"
                + "1234 6789\n"
                + "12345 789\n"
                + "123456 89\n"
                + "1234567 9\n"
                + "12345678 \n"
        );
    }
}
