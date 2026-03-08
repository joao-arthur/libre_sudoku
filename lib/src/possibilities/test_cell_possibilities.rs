use super::cell_possibilities;
use crate::{cell::Cell, group};

#[test]
fn _0_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("351672984"),
            &group::from_str("324519687"),
            &group::from_str("351287469"),
        ),
        vec![]
    );
}

#[test]
fn _0_possible_row() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("2875496 3"),
            &group::from_str("586724913"),
            &group::from_str("351287469"),
        ),
        vec![]
    );
}

#[test]
fn _0_possible_col() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("287549613"),
            &group::from_str("5867249 3"),
            &group::from_str("351287469"),
        ),
        vec![]
    );
}

#[test]
fn _0_possible_sq() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("287549613"),
            &group::from_str("586724913"),
            &group::from_str("35 287469"),
        ),
        vec![]
    );
}

#[test]
fn _1_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("2875496 3"),
            &group::from_str("5867249 3"),
            &group::from_str("35 287469"),
        ),
        vec![Cell::_1]
    );
}

#[test]
fn _2_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("46983  57"),
            &group::from_str(" 79386 54"),
            &group::from_str("35  87469"),
        ),
        vec![Cell::_1, Cell::_2]
    );
}

#[test]
fn _3_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("57  84 96"),
            &group::from_str("658   479"),
            &group::from_str(" 84 96 57"),
        ),
        vec![Cell::_1, Cell::_2, Cell::_3]
    );
}

#[test]
fn _4_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("  8 967 5"),
            &group::from_str("7  895 6 "),
            &group::from_str(" 8  96 57"),
        ),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4]
    );
}

#[test]
fn _5_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("9 6  78  "),
            &group::from_str(" 9  67  8"),
            &group::from_str(" 8  96  7"),
        ),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5]
    );
}

#[test]
fn _6_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str(" 9     78"),
            &group::from_str("9   78   "),
            &group::from_str(" 78  9   "),
        ),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6]
    );
}

#[test]
fn _7_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("8       9"),
            &group::from_str("8  9     "),
            &group::from_str("  8  9   "),
        ),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6, Cell::_7]
    );
}

#[test]
fn _8_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("   9     "),
            &group::from_str("       9 "),
            &group::from_str("     9   "),
        ),
        vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4, Cell::_5, Cell::_6, Cell::_7, Cell::_8]
    );
}

fn _9_possible() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("         "),
            &group::from_str("         "),
            &group::from_str("         "),
        ),
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
