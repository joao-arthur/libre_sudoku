use crate::{cell::Cell, group::Group};

fn strategy_last_empty_in_group(g: &Group) -> Option<Cell> {
    let cells: Vec<Cell> = g.iter().filter(|c| c.is_some()).map(|c| c.clone().unwrap()).collect();
    if cells.len() < 8 {
        return None;
    }
    let group = vec![
        Cell::_1,
        Cell::_2,
        Cell::_3,
        Cell::_4,
        Cell::_5,
        Cell::_6,
        Cell::_7,
        Cell::_8,
        Cell::_9,
    ];
    group.iter().cloned().filter(|gc| cells.iter().cloned().find(|c| c == gc).is_none()).last()
}

fn get_col_possibilities() {}
fn get_row_possibilities() {}
fn get_square_possibilities() {}

fn get_col_with_mandatory_posibility() {}
fn get_row_with_mandatory_posibility() {}
fn get_square_with_mandatory_posibility() {}

fn strategy_only_possibility() {}
fn strategy_last_possibility() {}

fn possibility_clear_pairs_row() {}
fn possibility_clear_pairs_col() {}
fn possibility_clear_pairs_square() {}

fn possibility_clear_trios_row() {}
fn possibility_clear_trios_col() {}
fn possibility_clear_trios_square() {}

#[cfg(test)]
mod test {
    use crate::group;

    use super::*;

    #[test]
    fn test_strategy_last_empty_in_group_some() {
        assert_eq!(strategy_last_empty_in_group(&group::from_str(" 23456789")), Some(Cell::_1));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1 3456789")), Some(Cell::_2));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12 456789")), Some(Cell::_3));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123 56789")), Some(Cell::_4));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234 6789")), Some(Cell::_5));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345 789")), Some(Cell::_6));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123456 89")), Some(Cell::_7));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234567 9")), Some(Cell::_8));
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345678 ")), Some(Cell::_9));
    }

    #[test]
    fn test_strategy_last_empty_in_group_none() {
        assert_eq!(strategy_last_empty_in_group(&group::from_str("  3456789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12  56789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234  789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123456  9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("   456789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123   789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123456   ")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("    56789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345    ")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("         ")), None);
    }
}
