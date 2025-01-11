use std::char;

use crate::cell::Cell;

pub type Group = [Option<Cell>; 9];

fn parse_digit(c: Option<char>) -> Option<Cell> {
    if let Some(c) = c {
        if c.is_digit(10) {
            return Cell::from_str(&c.to_string());
        } else {
            return None;
        }
    }
    None
}

pub fn from_str(row: &str) -> Group {
    [
        parse_digit(row.chars().nth(0)),
        parse_digit(row.chars().nth(1)),
        parse_digit(row.chars().nth(2)),
        parse_digit(row.chars().nth(3)),
        parse_digit(row.chars().nth(4)),
        parse_digit(row.chars().nth(5)),
        parse_digit(row.chars().nth(6)),
        parse_digit(row.chars().nth(7)),
        parse_digit(row.chars().nth(8)),
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_col() {
        assert_eq!(from_str("         "), [None, None, None, None, None, None, None, None, None]);
        assert_eq!(from_str(" 11111111"), [None, Some(Cell::_1), Some(Cell::_1), Some(Cell::_1), Some(Cell::_1), Some(Cell::_1), Some(Cell::_1), Some(Cell::_1), Some(Cell::_1)]);
        assert_eq!(from_str("2 2222222"), [Some(Cell::_2), None, Some(Cell::_2), Some(Cell::_2), Some(Cell::_2), Some(Cell::_2), Some(Cell::_2), Some(Cell::_2), Some(Cell::_2)]);
        assert_eq!(from_str("33 333333"), [Some(Cell::_3), Some(Cell::_3), None, Some(Cell::_3), Some(Cell::_3), Some(Cell::_3), Some(Cell::_3), Some(Cell::_3), Some(Cell::_3)]);
        assert_eq!(from_str("444 44444"), [Some(Cell::_4), Some(Cell::_4), Some(Cell::_4), None, Some(Cell::_4), Some(Cell::_4), Some(Cell::_4), Some(Cell::_4), Some(Cell::_4)]);
        assert_eq!(from_str("5555 5555"), [Some(Cell::_5), Some(Cell::_5), Some(Cell::_5), Some(Cell::_5), None, Some(Cell::_5), Some(Cell::_5), Some(Cell::_5), Some(Cell::_5)]);
        assert_eq!(from_str("66666 666"), [Some(Cell::_6), Some(Cell::_6), Some(Cell::_6), Some(Cell::_6), Some(Cell::_6), None, Some(Cell::_6), Some(Cell::_6), Some(Cell::_6)]);
        assert_eq!(from_str("777777 77"), [Some(Cell::_7), Some(Cell::_7), Some(Cell::_7), Some(Cell::_7), Some(Cell::_7), Some(Cell::_7), None, Some(Cell::_7), Some(Cell::_7)]);
        assert_eq!(from_str("8888888 8"), [Some(Cell::_8), Some(Cell::_8), Some(Cell::_8), Some(Cell::_8), Some(Cell::_8), Some(Cell::_8), Some(Cell::_8), None, Some(Cell::_8)]);
        assert_eq!(from_str("99999999 "), [Some(Cell::_9), Some(Cell::_9), Some(Cell::_9), Some(Cell::_9), Some(Cell::_9), Some(Cell::_9), Some(Cell::_9), Some(Cell::_9), None]);
    }
}
