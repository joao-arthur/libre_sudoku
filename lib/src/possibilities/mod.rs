use crate::{cell::Cell, group::Group};

pub type Possibilities = Vec<Cell>;

pub fn cell_possibilities(row: &Group, col: &Group, sq: &Group) -> Possibilities {
    let mut group_possibilities = [true; 9];
    for cell in row.iter().flatten() {
        group_possibilities[cell.to_usize()] = false;
    }
    for cell in col.iter().flatten() {
        group_possibilities[cell.to_usize()] = false;
    }
    for cell in sq.iter().flatten() {
        group_possibilities[cell.to_usize()] = false;
    }
    let mut possibilities: Possibilities = vec![];
    for (i, p) in group_possibilities.iter().enumerate() {
        if *p && let Some(c) = Cell::try_from_usize(i) {
            possibilities.push(c);
        }
    }
    possibilities
}

fn group_possibilities(group: &Group) -> Possibilities {
    let mut group_possibilities = [true; 9];
    for cell in group.iter().flatten() {
        group_possibilities[cell.to_usize()] = false;
    }
    let mut possibilities: Possibilities = vec![];
    for (i, p) in group_possibilities.iter().enumerate() {
        if *p && let Some(c) = Cell::try_from_usize(i) {
            possibilities.push(c);
        }
    }
    possibilities
}

#[cfg(test)]
mod test_cell_possibilities;

#[cfg(test)]
mod test_group_possibilities;
