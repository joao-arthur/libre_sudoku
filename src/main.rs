use std::{cell, collections::HashSet};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq,Eq, Clone, Hash)]
enum Cell {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

impl Cell {
    fn to_u8(&self) -> u8 {
        match self {
            Cell::_1 => 1,
            Cell::_2 => 2,
            Cell::_3 => 3,
            Cell::_4 => 4,
            Cell::_5 => 5,
            Cell::_6 => 6,
            Cell::_7 => 7,
            Cell::_8 => 8,
            Cell::_9 => 9,  
        }
    }
}

type Group = [Option<Cell>;9];
type Board = [[Option<Cell>;9]; 9];

fn board_from_str(rows: [&str; 9]) -> Board {
    
}

fn group_from_str(row: &str) -> Group {

}

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

fn get_col(b: &Board, i: &Cell) -> Group {
    [
        b.get(0).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(1).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(2).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(3).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(4).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(5).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(6).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(7).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
        b.get(8).unwrap().get(usize::from(i.to_u8() - 1)).unwrap().clone(),
    ]
}

fn get_row(b: &Board, i: &Cell) -> Group {
    b.get(usize::from(i.to_u8() - 1)).unwrap().clone()
}

fn get_sqr(b: &Board, i: &Cell) -> Group {
    let ii = i.to_u8() -1;
    let row_i = ii / 3;
    let col_i = ii - row_i * 3;

    [
        b.get(usize::from(row_i * 3 + 0)).unwrap().get(usize::from(col_i * 3 + 0)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 0)).unwrap().get(usize::from(col_i * 3 + 1)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 0)).unwrap().get(usize::from(col_i * 3 + 2)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 1)).unwrap().get(usize::from(col_i * 3 + 0)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 1)).unwrap().get(usize::from(col_i * 3 + 1)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 1)).unwrap().get(usize::from(col_i * 3 + 2)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 2)).unwrap().get(usize::from(col_i * 3 + 0)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 2)).unwrap().get(usize::from(col_i * 3 + 1)).unwrap().clone(),
        b.get(usize::from(row_i * 3 + 2)).unwrap().get(usize::from(col_i * 3 + 2)).unwrap().clone(),
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
    fn test_get_col() {
        let b = board_from_str([
            " 23456789",
            "1 3456789",
            "12 456789",
            "123 56789",
            "1234 6789",
            "12345 789",
            "123456 89",
            "1234567 9",
            "12345678 ",
        ]);
        assert_eq!(get_col(&b, &Cell::_1), group_from_str(" 11111111"));
        assert_eq!(get_col(&b, &Cell::_2), group_from_str("2 2222222"));
        assert_eq!(get_col(&b, &Cell::_3), group_from_str("33 333333"));
        assert_eq!(get_col(&b, &Cell::_4), group_from_str("444 44444"));
        assert_eq!(get_col(&b, &Cell::_5), group_from_str("5555 5555"));
        assert_eq!(get_col(&b, &Cell::_6), group_from_str("66666 666"));
        assert_eq!(get_col(&b, &Cell::_7), group_from_str("777777 77"));
        assert_eq!(get_col(&b, &Cell::_8), group_from_str("8888888 8"));
        assert_eq!(get_col(&b, &Cell::_9), group_from_str("99999999 "));
    }

    #[test]
    fn test_get_row() {
        let b = board_from_str([
            " 11111111",
            "2 2222222",
            "33 333333",
            "444 44444",
            "5555 5555",
            "66666 666",
            "777777 77",
            "8888888 8",
            "99999999 ",
        ]);
        assert_eq!(get_row(&b, &Cell::_1), group_from_str(" 11111111"));
        assert_eq!(get_row(&b, &Cell::_2), group_from_str("2 2222222"));
        assert_eq!(get_row(&b, &Cell::_3), group_from_str("33 333333"));
        assert_eq!(get_row(&b, &Cell::_4), group_from_str("444 44444"));
        assert_eq!(get_row(&b, &Cell::_5), group_from_str("5555 5555"));
        assert_eq!(get_row(&b, &Cell::_6), group_from_str("66666 666"));
        assert_eq!(get_row(&b, &Cell::_7), group_from_str("777777 77"));
        assert_eq!(get_row(&b, &Cell::_8), group_from_str("8888888 8"));
        assert_eq!(get_row(&b, &Cell::_9), group_from_str("99999999 "));
    }

    #[test]
    fn test_get_sqr() {
        let b = board_from_str([
            " 112 233 ",
            "111222333",
            "111222333",
            "444555666",
            " 445 566 ",
            "444555666",
            "777888999",
            "777888999",
            " 778 899 ",
        ]);
        assert_eq!(get_sqr(&b, &Cell::_1), group_from_str(" 11111111"));
        assert_eq!(get_sqr(&b, &Cell::_2), group_from_str("2 2222222"));
        assert_eq!(get_sqr(&b, &Cell::_3), group_from_str("33 333333"));
        assert_eq!(get_sqr(&b, &Cell::_4), group_from_str("444 44444"));
        assert_eq!(get_sqr(&b, &Cell::_5), group_from_str("5555 5555"));
        assert_eq!(get_sqr(&b, &Cell::_6), group_from_str("66666 666"));
        assert_eq!(get_sqr(&b, &Cell::_7), group_from_str("777777 77"));
        assert_eq!(get_sqr(&b, &Cell::_8), group_from_str("8888888 8"));
        assert_eq!(get_sqr(&b, &Cell::_9), group_from_str("99999999 "));
    }

    #[test]
    fn test_strategy_last_empty_in_group() {
        assert_eq!(strategy_last_empty_in_group(&group_from_str("12345678 ")), Some(Cell::_9));
    }
}