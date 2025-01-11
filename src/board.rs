use std::{collections::HashSet, fmt};

use crate::{
    cell::Cell,
    group::{self, Group},
};

pub type Board = [[Option<Cell>; 9]; 9];

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Only [0-9] characters are allowed!")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The length of every line must be 9!")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn from_str(rows: [&str; 9]) -> Result<Board, FromStringErr> {
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
    Ok([
        group::from_str(rows.get(0).unwrap()),
        group::from_str(rows.get(1).unwrap()),
        group::from_str(rows.get(2).unwrap()),
        group::from_str(rows.get(3).unwrap()),
        group::from_str(rows.get(4).unwrap()),
        group::from_str(rows.get(5).unwrap()),
        group::from_str(rows.get(6).unwrap()),
        group::from_str(rows.get(7).unwrap()),
        group::from_str(rows.get(8).unwrap()),
    ])
}

fn get_col(b: &Board, i: &Cell) -> Group {
    [
        b.get(0).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(1).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(2).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(3).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(4).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(5).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(6).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(7).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(8).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
    ]
}

fn get_row(b: &Board, i: &Cell) -> Group {
    b.get(usize::from(i.to_u8() - 1)).unwrap().clone()
}

fn get_sq(b: &Board, i: &Cell) -> Group {
    let ii = i.to_u8() - 1;
    let row_i = ii / 3;
    let col_i = ii - row_i * 3;

    [
        b.get(usize::from(row_i * 3 + 0)).unwrap().get(usize::from(col_i * 3 + 0)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 0)).unwrap().get(usize::from(col_i * 3 + 1)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 0)).unwrap().get(usize::from(col_i * 3 + 2)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 1)).unwrap().get(usize::from(col_i * 3 + 0)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 1)).unwrap().get(usize::from(col_i * 3 + 1)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 1)).unwrap().get(usize::from(col_i * 3 + 2)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 2)).unwrap().get(usize::from(col_i * 3 + 0)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 2)).unwrap().get(usize::from(col_i * 3 + 1)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 2)).unwrap().get(usize::from(col_i * 3 + 2)).unwrap().clone(),
    ]
}

#[cfg(test)]
mod test {
    use super::*;

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
            Ok([
                [None, Some(Cell::_2), Some(Cell::_3), Some(Cell::_4), Some(Cell::_5), Some(Cell::_6), Some(Cell::_7), Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), None, Some(Cell::_3), Some(Cell::_4), Some(Cell::_5), Some(Cell::_6), Some(Cell::_7), Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), None, Some(Cell::_4), Some(Cell::_5), Some(Cell::_6), Some(Cell::_7), Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), Some(Cell::_3), None, Some(Cell::_5), Some(Cell::_6), Some(Cell::_7), Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), Some(Cell::_3), Some(Cell::_4), None, Some(Cell::_6), Some(Cell::_7), Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), Some(Cell::_3), Some(Cell::_4), Some(Cell::_5), None, Some(Cell::_7), Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), Some(Cell::_3), Some(Cell::_4), Some(Cell::_5), Some(Cell::_6), None, Some(Cell::_8), Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), Some(Cell::_3), Some(Cell::_4), Some(Cell::_5), Some(Cell::_6), Some(Cell::_7), None, Some(Cell::_9)],
                [Some(Cell::_1), Some(Cell::_2), Some(Cell::_3), Some(Cell::_4), Some(Cell::_5), Some(Cell::_6), Some(Cell::_7), Some(Cell::_8), None],
            ])
        );
    }

    #[test]
    fn test_get_col() {
        let b = from_str([
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
        .unwrap();
        assert_eq!(get_col(&b, &Cell::_1), group::from_str(" 11111111"));
        assert_eq!(get_col(&b, &Cell::_2), group::from_str("2 2222222"));
        assert_eq!(get_col(&b, &Cell::_3), group::from_str("33 333333"));
        assert_eq!(get_col(&b, &Cell::_4), group::from_str("444 44444"));
        assert_eq!(get_col(&b, &Cell::_5), group::from_str("5555 5555"));
        assert_eq!(get_col(&b, &Cell::_6), group::from_str("66666 666"));
        assert_eq!(get_col(&b, &Cell::_7), group::from_str("777777 77"));
        assert_eq!(get_col(&b, &Cell::_8), group::from_str("8888888 8"));
        assert_eq!(get_col(&b, &Cell::_9), group::from_str("99999999 "));
    }

    #[test]
    fn test_get_row() {
        let b = from_str([
            " 11111111",
            "2 2222222",
            "33 333333",
            "444 44444",
            "5555 5555",
            "66666 666",
            "777777 77",
            "8888888 8",
            "99999999 ",
        ])
        .unwrap();
        assert_eq!(get_row(&b, &Cell::_1), group::from_str(" 11111111"));
        assert_eq!(get_row(&b, &Cell::_2), group::from_str("2 2222222"));
        assert_eq!(get_row(&b, &Cell::_3), group::from_str("33 333333"));
        assert_eq!(get_row(&b, &Cell::_4), group::from_str("444 44444"));
        assert_eq!(get_row(&b, &Cell::_5), group::from_str("5555 5555"));
        assert_eq!(get_row(&b, &Cell::_6), group::from_str("66666 666"));
        assert_eq!(get_row(&b, &Cell::_7), group::from_str("777777 77"));
        assert_eq!(get_row(&b, &Cell::_8), group::from_str("8888888 8"));
        assert_eq!(get_row(&b, &Cell::_9), group::from_str("99999999 "));
    }

    #[test]
    fn test_get_sq() {
        let b = from_str([
            " 112 233 ",
            "111222333",
            "111222333",
            "444555666",
            " 445 566 ",
            "444555666",
            "777888999",
            "777888999",
            " 778 899 ",
        ])
        .unwrap();
        assert_eq!(get_sq(&b, &Cell::_1), group::from_str(" 11111111"));
        assert_eq!(get_sq(&b, &Cell::_2), group::from_str("2 2222222"));
        assert_eq!(get_sq(&b, &Cell::_3), group::from_str("33 333333"));
        assert_eq!(get_sq(&b, &Cell::_4), group::from_str("444 44444"));
        assert_eq!(get_sq(&b, &Cell::_5), group::from_str("5555 5555"));
        assert_eq!(get_sq(&b, &Cell::_6), group::from_str("66666 666"));
        assert_eq!(get_sq(&b, &Cell::_7), group::from_str("777777 77"));
        assert_eq!(get_sq(&b, &Cell::_8), group::from_str("8888888 8"));
        assert_eq!(get_sq(&b, &Cell::_9), group::from_str("99999999 "));
    }
}
