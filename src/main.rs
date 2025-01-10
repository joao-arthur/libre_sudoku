mod board;
mod cell;
mod group;

fn main() {
    println!("Hello, world!");
}

/*

fn get_board() -> Board {
    return [
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, Some(Cell::_1), None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None],
    ]
}

fn strategy_last_empty_in_group(g: &Group) -> Option<Cell> {
    let cells: Vec<Cell> = g.iter().filter(|c| c.is_some()).map(|c| c.clone().unwrap()).collect();
    if cells.len() < 8 {
        return None;
    }
    let group = vec![
        Cell::_1,
        Cell::_2,
        Cell::_3,
        Cell::_4,
        Cell::_5,
        Cell::_6,
        Cell::_7,
        Cell::_8,
        Cell::_9,
    ];
    group.iter().cloned().filter(|gc| cells.iter().cloned().find(|c| c == gc).is_some()).last()
}

fn get_col_possibilities() { }
fn get_row_possibilities() { }
fn get_square_possibilities() { }

fn get_col_with_mandatory_posibility() { }
fn get_row_with_mandatory_posibility() { }
fn get_square_with_mandatory_posibility() { }


fn strategy_last_in_col() { }
fn strategy_last_in_square() { }

fn strategy_only_possibility() { }
fn strategy_last_possibility() { }

fn possibility_clear_pairs_row() { }
fn possibility_clear_pairs_col() { }
fn possibility_clear_pairs_square() { }

fn possibility_clear_trios_row() { }
fn possibility_clear_trios_col() { }
fn possibility_clear_trios_square() { }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strategy_last_empty_in_group() {
        assert_eq!(strategy_last_empty_in_group(&group_from_str("12345678 ")), Some(Cell::_9));
    }
}*/
