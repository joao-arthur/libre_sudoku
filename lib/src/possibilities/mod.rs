use crate::{cell::Cell, group::Group};

pub type Possibilities = Vec<Cell>;

pub fn cell_possibilities(row: &Group, col: &Group, sq: &Group) -> Possibilities {
    let mut group_possibilities = [true; 9];
    for c in row.iter().flatten() {
        group_possibilities[c.to_idx() as usize] = false;
    }
    for c in col.iter().flatten() {
        group_possibilities[c.to_idx() as usize] = false;
    }
    for c in sq.iter().flatten() {
        group_possibilities[c.to_idx() as usize] = false;
    }
    let mut possibilities: Possibilities = vec![];
    for (i, p) in group_possibilities.iter().enumerate() {
        if *p && let Some(c) = Cell::try_from_idx(i as u8) {
            possibilities.push(c);
        }
    }
    possibilities
}

fn group_possibilities(group: &Group) -> Possibilities {
    let mut group_possibilities = [true; 9];
    for c in group.iter().flatten() {
        group_possibilities[c.to_idx() as usize] = false;
    }
    let mut possibilities: Possibilities = vec![];
    for (i, p) in group_possibilities.iter().enumerate() {
        if *p && let Some(c) = Cell::try_from_idx(i as u8) {
            possibilities.push(c);
        }
    }
    possibilities
}

#[cfg(test)]
mod test_cell_possibilities;

#[cfg(test)]
mod test_group_possibilities;
