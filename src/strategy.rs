use crate::{cell::Cell, group::Group, possibilities::Possibilities};

pub fn strategy_last_empty_in_group(g: &Group) -> Option<Cell> {
    let mut cells_len: u8 = 0;
    for c in g {
        if c.is_some() {
            cells_len += 1;
        }
    }
    if cells_len != 8 {
        return None;
    }
    let mut group_taken = vec![false; 9];
    for c in g {
        if let Some(c) = c {
            group_taken[(c.to_idx()) as usize] = true;
        }
    }
    for (i, c) in group_taken.iter().enumerate() {
        if !c {
            return Cell::from_idx(i as u8);
        }
    }
    None
}

pub fn strategy_only_possibility(p: &Possibilities) -> Option<Cell> {
    if p.len() == 1 {
        return p.get(0).cloned();
    }
    None
}

fn get_col_with_mandatory_posibility() {}
fn get_row_with_mandatory_posibility() {}
fn get_square_with_mandatory_posibility() {}

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

    fn strategy_only_possibility_some() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1]), Some(Cell::_1));
        assert_eq!(strategy_only_possibility(&vec![Cell::_2]), Some(Cell::_2));
        assert_eq!(strategy_only_possibility(&vec![Cell::_3]), Some(Cell::_3));
        assert_eq!(strategy_only_possibility(&vec![Cell::_4]), Some(Cell::_4));
        assert_eq!(strategy_only_possibility(&vec![Cell::_5]), Some(Cell::_5));
        assert_eq!(strategy_only_possibility(&vec![Cell::_6]), Some(Cell::_6));
        assert_eq!(strategy_only_possibility(&vec![Cell::_7]), Some(Cell::_7));
        assert_eq!(strategy_only_possibility(&vec![Cell::_8]), Some(Cell::_8));
        assert_eq!(strategy_only_possibility(&vec![Cell::_9]), Some(Cell::_9));
    }

    fn strategy_only_possibility_none() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2]), None);
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2, Cell::_3]), None);
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4]), None);
    }
}
