use std::collections::LinkedList;

use crate::{cell::Cell, group::Group};

fn strategy_last_empty_in_group(g: &Group) -> Option<Cell> {
    let mut cells_len: u8 = 0;
    for c in g {
        if c.is_some() {
            cells_len += 1;
        }
    }
    if cells_len != 8 {
        return None;
    }
    let mut group = vec![false; 9];
    for c in g {
        if let Some(c) = c {
            group[(c.to_u8() - 1) as usize] = true;
        }
    }
    for (i, c) in group.iter().enumerate() {
        if !c {
            return Cell::from_u8((i + 1) as u8);
        }
    }
    None
}

fn get_col_possibilities() {}
fn get_row_possibilities() {}
fn get_square_possibilities() {}

fn get_possibilities(
    row: Group,
    col: Group,
    sq: Group,
    x: u8,
    y: u8
) {




}

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
    use std::time::Instant;

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
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1  456789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12  56789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123  6789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234  789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345  89")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123456  9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234567  ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("   456789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1   56789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12   6789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123   789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234   89")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345   9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123456   ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("    56789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1    6789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12    789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123    89")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1234    9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345    ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("     6789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1     789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12     89")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123     9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12345    ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("      789")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1      89")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12      9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("123      ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("       89")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1       9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("12       ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("        9")), None);
        assert_eq!(strategy_last_empty_in_group(&group::from_str("1        ")), None);

        assert_eq!(strategy_last_empty_in_group(&group::from_str("         ")), None);
    }
}
