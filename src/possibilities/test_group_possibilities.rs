use super::group_possibilities;
use crate::{cell::Cell, group};

#[test]
fn _0_possible() {
    assert_eq!(group_possibilities(&group::from_str("351672984")), vec![]);
}

#[test]
fn _1_possible() {
    assert_eq!(group_possibilities(&group::from_str("2875496 3")), vec![Cell::_1]);
}

#[test]
fn _2_possible() {
    assert_eq!(group_possibilities(&group::from_str("46983  57")), vec![Cell::_1, Cell::_2]);
}

#[test]
fn _3_possible() {
    assert_eq!(
        group_possibilities(&group::from_str("57  84 96")),
        vec![Cell::_1, Cell::_2, Cell::_3]
    );
}

#[test]
fn _4_possible() {
    assert_eq!(
        group_possibilities(&group::from_str("  8 967 5")),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4]
    );
}

#[test]
fn _5_possible() {
    assert_eq!(
        group_possibilities(&group::from_str("9 6  78  ")),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5]
    );
}

#[test]
fn _6_possible() {
    assert_eq!(
        group_possibilities(&group::from_str(" 9     78")),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6]
    );
}

#[test]
fn _7_possible() {
    assert_eq!(
        group_possibilities(&group::from_str("8       9")),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6, Cell::_7]
    );
}

#[test]
fn _8_possible() {
    assert_eq!(
        group_possibilities(&group::from_str("   9     ")),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6, Cell::_7, Cell::_8]
    );
}

fn _9_possible() {
    assert_eq!(
        group_possibilities(&group::from_str("         ")),
        vec![
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
}
