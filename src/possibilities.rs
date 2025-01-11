use crate::{cell::Cell, group::Group};

pub type Possibilities = Vec<Cell>;

fn get_col_possibilities() {}
fn get_row_possibilities() {}
fn get_square_possibilities() {}

fn get_possibilities(row: &Group, col: &Group, sq: &Group) -> Possibilities {
    let mut group_possibilities = vec![true; 9];
    for c in row {
        if let Some(c) = c {
            group_possibilities[(c.to_idx()) as usize] = false;
        }
    }
    for c in col {
        if let Some(c) = c {
            group_possibilities[(c.to_idx()) as usize] = false;
        }
    }
    for c in sq {
        if let Some(c) = c {
            group_possibilities[(c.to_idx()) as usize] = false;
        }
    }
    let mut possibilities: Possibilities = vec![];
    for (i, p) in group_possibilities.iter().enumerate() {
        if *p {
            if let Some(c) = Cell::from_idx(i as u8) {
                possibilities.push(c);
            }
        }
    }
    possibilities
}

#[cfg(test)]
mod test {
    use crate::group;

    use super::*;

    #[test]
    fn test_get_possibilities() {
        assert_eq!(
            get_possibilities(
                &group::from_str("    9  3 "),
                &group::from_str("  4 6   9"),
                &group::from_str("   726 9 "),
            ),
            vec![Cell::_1, Cell::_5, Cell::_8]
        );
    }
}
