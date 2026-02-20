use crate::solved_group::{SolvedGroup, solved_group_from_str};
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

pub fn try_solved_board_from_str(rows: [&str; 9]) -> Result<SolvedBoard, FromStringErr> {
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
            solved_group_from_str(rows.get_unchecked(0)),
            solved_group_from_str(rows.get_unchecked(1)),
            solved_group_from_str(rows.get_unchecked(2)),
            solved_group_from_str(rows.get_unchecked(3)),
            solved_group_from_str(rows.get_unchecked(4)),
            solved_group_from_str(rows.get_unchecked(5)),
            solved_group_from_str(rows.get_unchecked(6)),
            solved_group_from_str(rows.get_unchecked(7)),
            solved_group_from_str(rows.get_unchecked(8)),
        ])
    }
}

pub fn solved_board_from_str(rows: [&str; 9]) -> SolvedBoard {
    try_solved_board_from_str(rows).unwrap()
}

pub fn solved_board_to_string(board: &SolvedBoard) -> String {
    let mut res = String::from("");
    for row in board {
        for col in row {
            res.push_str(col.to_str());
        }
        res.push('\n')
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{
        FromStringErr,
        InvalidCharacterErr,
        InvalidLengthErr,
        solved_board_from_str,
        //get_cell,
        //get_col,
        //get_row,
        //get_sq,
        //get_sq_idx,
        solved_board_to_string,
        try_solved_board_from_str,
    };
    use crate::solved_group::solved_group_from_str;

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
            try_solved_board_from_str([
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
                solved_group_from_str("123456789"),
                solved_group_from_str("234567891"),
                solved_group_from_str("345678912"),
                solved_group_from_str("456789123"),
                solved_group_from_str("567891234"),
                solved_group_from_str("678912345"),
                solved_group_from_str("789123456"),
                solved_group_from_str("891234567"),
                solved_group_from_str("912345678"),
            ])
        );
    }

    #[test]
    fn try_from_str_invalid_character() {
        assert_eq!(
            try_solved_board_from_str([
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
            try_solved_board_from_str([
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
            try_solved_board_from_str([
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
            try_solved_board_from_str([
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
    fn to_string() {
        assert_eq!(
            solved_board_to_string(&solved_board_from_str([
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
}
