#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
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
    pub fn to_idx(&self) -> u8 {
        match self {
            Cell::_1 => 0,
            Cell::_2 => 1,
            Cell::_3 => 2,
            Cell::_4 => 3,
            Cell::_5 => 4,
            Cell::_6 => 5,
            Cell::_7 => 6,
            Cell::_8 => 7,
            Cell::_9 => 8,
        }
    }

    pub fn from_idx(value: u8) -> Option<Cell> {
        match value {
            0 => Some(Cell::_1),
            1 => Some(Cell::_2),
            2 => Some(Cell::_3),
            3 => Some(Cell::_4),
            4 => Some(Cell::_5),
            5 => Some(Cell::_6),
            6 => Some(Cell::_7),
            7 => Some(Cell::_8),
            8 => Some(Cell::_9),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Cell> {
        match s {
            "1" => Some(Cell::_1),
            "2" => Some(Cell::_2),
            "3" => Some(Cell::_3),
            "4" => Some(Cell::_4),
            "5" => Some(Cell::_5),
            "6" => Some(Cell::_6),
            "7" => Some(Cell::_7),
            "8" => Some(Cell::_8),
            "9" => Some(Cell::_9),
            _ => None,
        }
    }

    pub fn to_str(self: &Cell) -> &'static str {
        match self {
            Cell::_1 => "1",
            Cell::_2 => "2",
            Cell::_3 => "3",
            Cell::_4 => "4",
            Cell::_5 => "5",
            Cell::_6 => "6",
            Cell::_7 => "7",
            Cell::_8 => "8",
            Cell::_9 => "9",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_to_idx() {
        assert_eq!(Cell::_1.to_idx(), 0);
        assert_eq!(Cell::_2.to_idx(), 1);
        assert_eq!(Cell::_3.to_idx(), 2);
        assert_eq!(Cell::_4.to_idx(), 3);
        assert_eq!(Cell::_5.to_idx(), 4);
        assert_eq!(Cell::_6.to_idx(), 5);
        assert_eq!(Cell::_7.to_idx(), 6);
        assert_eq!(Cell::_8.to_idx(), 7);
        assert_eq!(Cell::_9.to_idx(), 8);
    }

    #[test]
    fn test_cell_from_idx() {
        assert_eq!(Cell::from_idx(0), Some(Cell::_1));
        assert_eq!(Cell::from_idx(1), Some(Cell::_2));
        assert_eq!(Cell::from_idx(2), Some(Cell::_3));
        assert_eq!(Cell::from_idx(3), Some(Cell::_4));
        assert_eq!(Cell::from_idx(4), Some(Cell::_5));
        assert_eq!(Cell::from_idx(5), Some(Cell::_6));
        assert_eq!(Cell::from_idx(6), Some(Cell::_7));
        assert_eq!(Cell::from_idx(7), Some(Cell::_8));
        assert_eq!(Cell::from_idx(8), Some(Cell::_9));
        assert_eq!(Cell::from_idx(9), None);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Cell::from_str("0"), None);
        assert_eq!(Cell::from_str("1"), Some(Cell::_1));
        assert_eq!(Cell::from_str("2"), Some(Cell::_2));
        assert_eq!(Cell::from_str("3"), Some(Cell::_3));
        assert_eq!(Cell::from_str("4"), Some(Cell::_4));
        assert_eq!(Cell::from_str("5"), Some(Cell::_5));
        assert_eq!(Cell::from_str("6"), Some(Cell::_6));
        assert_eq!(Cell::from_str("7"), Some(Cell::_7));
        assert_eq!(Cell::from_str("8"), Some(Cell::_8));
        assert_eq!(Cell::from_str("9"), Some(Cell::_9));
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Cell::_1.to_str(), "1");
        assert_eq!(Cell::_2.to_str(), "2");
        assert_eq!(Cell::_3.to_str(), "3");
        assert_eq!(Cell::_4.to_str(), "4");
        assert_eq!(Cell::_5.to_str(), "5");
        assert_eq!(Cell::_6.to_str(), "6");
        assert_eq!(Cell::_7.to_str(), "7");
        assert_eq!(Cell::_8.to_str(), "8");
        assert_eq!(Cell::_9.to_str(), "9");
    }
}
