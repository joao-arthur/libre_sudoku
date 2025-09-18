use crate::{cell::Cell, group::Group};

pub type Possibilities = Vec<Cell>;

pub use self::cell::cell_possibilities;

mod cell;

fn get_col_possibilities() {}
fn get_row_possibilities() {}
fn get_square_possibilities() {}
