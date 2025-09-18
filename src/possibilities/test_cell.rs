use super::cell_possibilities;
use crate::{cell::Cell, group};

#[test]
fn test_cell_possibilities() {
    assert_eq!(
        cell_possibilities(
            &group::from_str("    9  3 "),
            &group::from_str("  4 6   9"),
            &group::from_str("   726 9 "),
        ),
        vec![Cell::_1, Cell::_5, Cell::_8]
    );
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
