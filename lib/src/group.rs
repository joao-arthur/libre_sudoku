use crate::cell::Cell;
use std::char;

pub type Group = [Option<Cell>; 9];

pub fn group_from_str(row: &str) -> Group {
    let mut chars = row.chars();
    [
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
        chars.next().map_or(None, |c| Cell::try_from_char(&c)),
    ]
}

pub fn group_to_string(group: &Group) -> String {
    let mut res = String::from("");
    for col in group {
        match col {
            Some(val) => res.push_str(val.to_str()),
            None => res.push_str(" "),
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{group_from_str, group_to_string};
    use crate::cell::Cell;

    #[test]
    fn group_from_empty_str() {
        assert_eq!(group_from_str(""), [None, None, None, None, None, None, None, None, None]);
        assert_eq!(
            group_from_str("         "),
            [None, None, None, None, None, None, None, None, None]
        );
    }

    #[test]
    fn group_from_missing_parts() {
        assert_eq!(
            group_from_str(" 11111111"),
            [
                None,
                Some(Cell::_1),
                Some(Cell::_1),
                Some(Cell::_1),
                Some(Cell::_1),
                Some(Cell::_1),
                Some(Cell::_1),
                Some(Cell::_1),
                Some(Cell::_1)
            ]
        );
        assert_eq!(
            group_from_str("2 2222222"),
            [
                Some(Cell::_2),
                None,
                Some(Cell::_2),
                Some(Cell::_2),
                Some(Cell::_2),
                Some(Cell::_2),
                Some(Cell::_2),
                Some(Cell::_2),
                Some(Cell::_2)
            ]
        );
        assert_eq!(
            group_from_str("33 333333"),
            [
                Some(Cell::_3),
                Some(Cell::_3),
                None,
                Some(Cell::_3),
                Some(Cell::_3),
                Some(Cell::_3),
                Some(Cell::_3),
                Some(Cell::_3),
                Some(Cell::_3)
            ]
        );
        assert_eq!(
            group_from_str("444 44444"),
            [
                Some(Cell::_4),
                Some(Cell::_4),
                Some(Cell::_4),
                None,
                Some(Cell::_4),
                Some(Cell::_4),
                Some(Cell::_4),
                Some(Cell::_4),
                Some(Cell::_4)
            ]
        );
        assert_eq!(
            group_from_str("5555 5555"),
            [
                Some(Cell::_5),
                Some(Cell::_5),
                Some(Cell::_5),
                Some(Cell::_5),
                None,
                Some(Cell::_5),
                Some(Cell::_5),
                Some(Cell::_5),
                Some(Cell::_5)
            ]
        );
        assert_eq!(
            group_from_str("66666 666"),
            [
                Some(Cell::_6),
                Some(Cell::_6),
                Some(Cell::_6),
                Some(Cell::_6),
                Some(Cell::_6),
                None,
                Some(Cell::_6),
                Some(Cell::_6),
                Some(Cell::_6)
            ]
        );
        assert_eq!(
            group_from_str("777777 77"),
            [
                Some(Cell::_7),
                Some(Cell::_7),
                Some(Cell::_7),
                Some(Cell::_7),
                Some(Cell::_7),
                Some(Cell::_7),
                None,
                Some(Cell::_7),
                Some(Cell::_7)
            ]
        );
        assert_eq!(
            group_from_str("8888888 8"),
            [
                Some(Cell::_8),
                Some(Cell::_8),
                Some(Cell::_8),
                Some(Cell::_8),
                Some(Cell::_8),
                Some(Cell::_8),
                Some(Cell::_8),
                None,
                Some(Cell::_8)
            ]
        );
        assert_eq!(
            group_from_str("99999999 "),
            [
                Some(Cell::_9),
                Some(Cell::_9),
                Some(Cell::_9),
                Some(Cell::_9),
                Some(Cell::_9),
                Some(Cell::_9),
                Some(Cell::_9),
                Some(Cell::_9),
                None
            ]
        );
    }

    #[test]
    fn group_from_full_str() {
        assert_eq!(
            group_from_str("123456789"),
            [
                Some(Cell::_1),
                Some(Cell::_2),
                Some(Cell::_3),
                Some(Cell::_4),
                Some(Cell::_5),
                Some(Cell::_6),
                Some(Cell::_7),
                Some(Cell::_8),
                Some(Cell::_9),
            ]
        );
    }

    #[test]
    fn test_group_to_string() {
        assert_eq!(group_to_string(&group_from_str("123456789")), "123456789");
        assert_eq!(group_to_string(&group_from_str("111111111")), "111111111");
        assert_eq!(group_to_string(&group_from_str("1 2 3 4 5")), "1 2 3 4 5");
    }
}
