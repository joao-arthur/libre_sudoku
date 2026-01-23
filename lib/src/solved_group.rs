use crate::cell::Cell;

pub type SolvedGroup = [Cell; 9];

pub fn try_solved_group_from_str(row: &str) -> Option<SolvedGroup> {
    let mut chars = row.chars();
    Some([
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
        Cell::try_from_char(&chars.next()?)?,
    ])
}

pub fn solved_group_from_str(row: &str) -> SolvedGroup {
    try_solved_group_from_str(row).unwrap()
}

pub fn solved_group_to_string(group: &SolvedGroup) -> String {
    let mut res = String::from("");
    for col in group {
        res.push_str(col.to_str())
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{solved_group_from_str, solved_group_to_string, try_solved_group_from_str};
    use crate::cell::Cell;

    #[test]
    fn try_solved_group_from_empty_str() {
        assert_eq!(try_solved_group_from_str(""), None);
        assert_eq!(try_solved_group_from_str("         "), None);
    }

    #[test]
    fn try_solved_group_from_missing_parts() {
        assert_eq!(try_solved_group_from_str(" 11111111"), None);
        assert_eq!(try_solved_group_from_str("2 2222222"), None);
        assert_eq!(try_solved_group_from_str("33 333333"), None);
        assert_eq!(try_solved_group_from_str("444 44444"), None);
        assert_eq!(try_solved_group_from_str("5555 5555"), None);
        assert_eq!(try_solved_group_from_str("66666 666"), None);
        assert_eq!(try_solved_group_from_str("777777 77"), None);
        assert_eq!(try_solved_group_from_str("8888888 8"), None);
        assert_eq!(try_solved_group_from_str("99999999 "), None);
    }

    #[test]
    fn try_solved_group_from_str_full_str() {
        assert_eq!(
            try_solved_group_from_str("123456789"),
            Some([
                Cell::_1,
                Cell::_2,
                Cell::_3,
                Cell::_4,
                Cell::_5,
                Cell::_6,
                Cell::_7,
                Cell::_8,
                Cell::_9
            ])
        );
        assert_eq!(
            try_solved_group_from_str("444444444"),
            Some([
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4
            ])
        );
    }

    #[test]
    fn solved_group_from_str_full_str() {
        assert_eq!(
            solved_group_from_str("123456789"),
            [
                Cell::_1,
                Cell::_2,
                Cell::_3,
                Cell::_4,
                Cell::_5,
                Cell::_6,
                Cell::_7,
                Cell::_8,
                Cell::_9
            ]
        );
        assert_eq!(
            solved_group_from_str("444444444"),
            [
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4,
                Cell::_4
            ]
        );
    }

    #[test]
    fn test_solved_group_to_string() {
        assert_eq!(solved_group_to_string(&solved_group_from_str("123456789")), "123456789");
        assert_eq!(solved_group_to_string(&solved_group_from_str("111111111")), "111111111");
    }
}
