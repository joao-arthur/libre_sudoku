use crate::{
    cell::Cell,
    solved_group::{self, SolvedGroup},
};
use std::fmt;

pub type SolvedBoard = [SolvedGroup; 9];

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Must match the pattern [0-9]")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Must have 9 lines with 9 characters each")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn try_from_str(rows: [&str; 9]) -> Result<SolvedBoard, FromStringErr> {
    for line in rows {
        if line.chars().count() != 9 {
            return Err(FromStringErr::InvalidLength(InvalidLengthErr));
        }
    }
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
        .is_empty()
    {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    unsafe {
        Ok([
            solved_group::from_str(rows.get_unchecked(0)),
            solved_group::from_str(rows.get_unchecked(1)),
            solved_group::from_str(rows.get_unchecked(2)),
            solved_group::from_str(rows.get_unchecked(3)),
            solved_group::from_str(rows.get_unchecked(4)),
            solved_group::from_str(rows.get_unchecked(5)),
            solved_group::from_str(rows.get_unchecked(6)),
            solved_group::from_str(rows.get_unchecked(7)),
            solved_group::from_str(rows.get_unchecked(8)),
        ])
    }
}

pub fn from_str(rows: [&str; 9]) -> SolvedBoard {
    try_from_str(rows).unwrap()
}

pub fn to_string(board: &SolvedBoard) -> String {
    let mut res = String::from("");
    for row in board {
        for col in row {
            res.push_str(col.to_str());
        }
        res.push('\n')
    }
    res
}

pub fn get_row(board: &SolvedBoard, cell: &Cell) -> SolvedGroup {
    unsafe { *board.get_unchecked(cell.to_usize()) }
}

pub fn get_col(board: &SolvedBoard, cell: &Cell) -> SolvedGroup {
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

pub fn get_sq_idx(row: &Cell, col: &Cell) -> Cell {
    let row_i = row.to_u8() / 3;
    let col_i = col.to_u8() / 3;
    Cell::from_u8(row_i * 3 + col_i)
}

pub fn get_sq(board: &SolvedBoard, cell: &Cell) -> SolvedGroup {
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

pub fn get_cell(board: &SolvedBoard, row: &Cell, col: &Cell) -> Cell {
    unsafe { *board.get_unchecked(row.to_usize()).get_unchecked(col.to_usize()) }
}

#[cfg(test)]
mod tests {
    use super::{
        FromStringErr, InvalidCharacterErr, InvalidLengthErr, from_str, get_cell, get_col, get_row,
        get_sq, get_sq_idx, to_string, try_from_str,
    };
    use crate::{cell::Cell, solved_group};

    #[test]
    fn invalid_character_err() {
        assert_eq!(InvalidCharacterErr.to_string(), "Must match the pattern [0-9]");
    }

    #[test]
    fn invalid_length_err() {
        assert_eq!(InvalidLengthErr.to_string(), "Must have 9 lines with 9 characters each");
    }

    #[test]
    fn try_from_str_ok() {
        assert_eq!(
            try_from_str([
                "123456789",
                "234567891",
                "345678912",
                "456789123",
                "567891234",
                "678912345",
                "789123456",
                "891234567",
                "912345678",
            ]),
            Ok([
                solved_group::from_str("123456789"),
                solved_group::from_str("234567891"),
                solved_group::from_str("345678912"),
                solved_group::from_str("456789123"),
                solved_group::from_str("567891234"),
                solved_group::from_str("678912345"),
                solved_group::from_str("789123456"),
                solved_group::from_str("891234567"),
                solved_group::from_str("912345678"),
            ])
        );
    }

    #[test]
    fn try_from_str_invalid_character() {
        assert_eq!(
            try_from_str([
                "a23456789",
                "234567891",
                "345678912",
                "456789123",
                "567891234",
                "678912345",
                "789123456",
                "891234567",
                "912345678",
            ]),
            Err(FromStringErr::InvalidCharacter(InvalidCharacterErr))
        );
        assert_eq!(
            try_from_str([
                "123456789",
                "234567891",
                "345678912",
                "456789123",
                "567891234",
                "678912345",
                "789123456",
                "891234567",
                "91234567z",
            ]),
            Err(FromStringErr::InvalidCharacter(InvalidCharacterErr))
        );
    }

    #[test]
    fn try_from_str_invalid_line_characters_length() {
        assert_eq!(
            try_from_str([
                "23456789",
                "234567891",
                "345678912",
                "456789123",
                "567891234",
                "678912345",
                "789123456",
                "891234567",
                "912345678",
            ]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr))
        );
        assert_eq!(
            try_from_str([
                "123456789",
                "234567891",
                "345678912",
                "456789123",
                "567891234",
                "678912345",
                "789123456",
                "891234567",
                "91234567",
            ]),
            Err(FromStringErr::InvalidLength(InvalidLengthErr))
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            to_string(&from_str([
                "123456789",
                "234567891",
                "345678912",
                "456789123",
                "567891234",
                "678912345",
                "789123456",
                "891234567",
                "912345678",
            ])),
            String::from("123456789\n")
                + "234567891\n"
                + "345678912\n"
                + "456789123\n"
                + "567891234\n"
                + "678912345\n"
                + "789123456\n"
                + "891234567\n"
                + "912345678\n"
        );
    }

    #[test]
    fn test_get_row() {
        let board = from_str([
            "111111111",
            "222222222",
            "333333333",
            "444444444",
            "555555555",
            "666666666",
            "777777777",
            "888888888",
            "999999999",
        ]);
        assert_eq!(get_row(&board, &Cell::_1), solved_group::from_str("111111111"));
        assert_eq!(get_row(&board, &Cell::_2), solved_group::from_str("222222222"));
        assert_eq!(get_row(&board, &Cell::_3), solved_group::from_str("333333333"));
        assert_eq!(get_row(&board, &Cell::_4), solved_group::from_str("444444444"));
        assert_eq!(get_row(&board, &Cell::_5), solved_group::from_str("555555555"));
        assert_eq!(get_row(&board, &Cell::_6), solved_group::from_str("666666666"));
        assert_eq!(get_row(&board, &Cell::_7), solved_group::from_str("777777777"));
        assert_eq!(get_row(&board, &Cell::_8), solved_group::from_str("888888888"));
        assert_eq!(get_row(&board, &Cell::_9), solved_group::from_str("999999999"));
    }

    #[test]
    fn test_get_col() {
        let board = from_str([
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
        ]);
        assert_eq!(get_col(&board, &Cell::_1), solved_group::from_str("111111111"));
        assert_eq!(get_col(&board, &Cell::_2), solved_group::from_str("222222222"));
        assert_eq!(get_col(&board, &Cell::_3), solved_group::from_str("333333333"));
        assert_eq!(get_col(&board, &Cell::_4), solved_group::from_str("444444444"));
        assert_eq!(get_col(&board, &Cell::_5), solved_group::from_str("555555555"));
        assert_eq!(get_col(&board, &Cell::_6), solved_group::from_str("666666666"));
        assert_eq!(get_col(&board, &Cell::_7), solved_group::from_str("777777777"));
        assert_eq!(get_col(&board, &Cell::_8), solved_group::from_str("888888888"));
        assert_eq!(get_col(&board, &Cell::_9), solved_group::from_str("999999999"));
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
    fn test_get_sq() {
        let board = from_str([
            "111222333",
            "111222333",
            "111222333",
            "444555666",
            "444555666",
            "444555666",
            "777888999",
            "777888999",
            "777888999",
        ]);
        assert_eq!(get_sq(&board, &Cell::_1), solved_group::from_str("111111111"));
        assert_eq!(get_sq(&board, &Cell::_2), solved_group::from_str("222222222"));
        assert_eq!(get_sq(&board, &Cell::_3), solved_group::from_str("333333333"));
        assert_eq!(get_sq(&board, &Cell::_4), solved_group::from_str("444444444"));
        assert_eq!(get_sq(&board, &Cell::_5), solved_group::from_str("555555555"));
        assert_eq!(get_sq(&board, &Cell::_6), solved_group::from_str("666666666"));
        assert_eq!(get_sq(&board, &Cell::_7), solved_group::from_str("777777777"));
        assert_eq!(get_sq(&board, &Cell::_8), solved_group::from_str("888888888"));
        assert_eq!(get_sq(&board, &Cell::_9), solved_group::from_str("999999999"));
    }

    #[test]
    fn test_get_cell() {
        let board = from_str([
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
            "123456789",
        ]);
        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_1), Cell::_1);
        assert_eq!(get_cell(&board, &Cell::_2, &Cell::_2), Cell::_2);
        assert_eq!(get_cell(&board, &Cell::_3, &Cell::_3), Cell::_3);

        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_2), Cell::_2);
        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_3), Cell::_3);
        assert_eq!(get_cell(&board, &Cell::_1, &Cell::_4), Cell::_4);

        assert_eq!(get_cell(&board, &Cell::_9, &Cell::_6), Cell::_6);
        assert_eq!(get_cell(&board, &Cell::_9, &Cell::_7), Cell::_7);
        assert_eq!(get_cell(&board, &Cell::_9, &Cell::_8), Cell::_8);
    }
}
