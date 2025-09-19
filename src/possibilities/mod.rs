use crate::{cell::Cell, group::Group};

pub type Possibilities = Vec<Cell>;

pub fn cell_possibilities(row: &Group, col: &Group, sq: &Group) -> Possibilities {
    let mut group_possibilities = vec![true; 9];
    for c in row {
        if let Some(c) = c {
            group_possibilities[c.to_idx() as usize] = false;
        }
    }
    for c in col {
        if let Some(c) = c {
            group_possibilities[c.to_idx() as usize] = false;
        }
    }
    for c in sq {
        if let Some(c) = c {
            group_possibilities[c.to_idx() as usize] = false;
        }
    }
    let mut possibilities: Possibilities = vec![];
    for (i, p) in group_possibilities.iter().enumerate() {
        if *p {
            if let Some(c) = Cell::try_from_idx(i as u8) {
                possibilities.push(c);
            }
        }
    }
    possibilities
}

fn col_possibilities() -> Possibilities {
    vec![]
}

fn row_possibilities() -> Possibilities {
    vec![]
}

fn square_possibilities() -> Possibilities {
    vec![]
}

#[cfg(test)]
mod test_cell;

#[cfg(test)]
mod test_col;

#[cfg(test)]
mod test_row;

#[cfg(test)]
mod test_sq;
