use crate::{cell::Cell, possibilities::Possibilities};

pub fn strategy_only_possibility(p: &Possibilities) -> Option<Cell> {
    if p.len() == 1 {
        return p.get(0).cloned();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::strategy_only_possibility;
    use crate::cell::Cell;

    #[test]
    fn possible_1() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1]), Some(Cell::_1));
        assert_eq!(strategy_only_possibility(&vec![Cell::_2]), Some(Cell::_2));
        assert_eq!(strategy_only_possibility(&vec![Cell::_3]), Some(Cell::_3));
        assert_eq!(strategy_only_possibility(&vec![Cell::_4]), Some(Cell::_4));
        assert_eq!(strategy_only_possibility(&vec![Cell::_5]), Some(Cell::_5));
        assert_eq!(strategy_only_possibility(&vec![Cell::_6]), Some(Cell::_6));
        assert_eq!(strategy_only_possibility(&vec![Cell::_7]), Some(Cell::_7));
        assert_eq!(strategy_only_possibility(&vec![Cell::_8]), Some(Cell::_8));
        assert_eq!(strategy_only_possibility(&vec![Cell::_9]), Some(Cell::_9));
    }

    #[test]
    fn possible_2() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2]), None);
    }

    #[test]
    fn possible_3() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2, Cell::_3]), None);
    }

    #[test]
    fn possible_4() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4]), None);
    }

    #[test]
    fn possible_5() {
        assert_eq!(strategy_only_possibility(&vec![Cell::_1, Cell::_2, Cell::_3, Cell::_4]), None);
    }
}
