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
    pub fn to_u8(&self) -> u8 {
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

    pub fn from_u8(value: u8) -> Option<Cell> {
        match value {
            1 => Some(Cell::_1),
            2 => Some(Cell::_2),
            3 => Some(Cell::_3),
            4 => Some(Cell::_4),
            5 => Some(Cell::_5),
            6 => Some(Cell::_6),
            7 => Some(Cell::_7),
            8 => Some(Cell::_8),
            9 => Some(Cell::_9),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_to_u8() {
        assert_eq!(Cell::_1.to_u8(), 1);
        assert_eq!(Cell::_2.to_u8(), 2);
        assert_eq!(Cell::_3.to_u8(), 3);
        assert_eq!(Cell::_4.to_u8(), 4);
        assert_eq!(Cell::_5.to_u8(), 5);
        assert_eq!(Cell::_6.to_u8(), 6);
        assert_eq!(Cell::_7.to_u8(), 7);
        assert_eq!(Cell::_8.to_u8(), 8);
        assert_eq!(Cell::_9.to_u8(), 9);
    }

    #[test]
    fn test_cell_from_u8() {
        assert_eq!(Cell::from_u8(0), None);
        assert_eq!(Cell::from_u8(1), Some(Cell::_1));
        assert_eq!(Cell::from_u8(2), Some(Cell::_2));
        assert_eq!(Cell::from_u8(3), Some(Cell::_3));
        assert_eq!(Cell::from_u8(4), Some(Cell::_4));
        assert_eq!(Cell::from_u8(5), Some(Cell::_5));
        assert_eq!(Cell::from_u8(6), Some(Cell::_6));
        assert_eq!(Cell::from_u8(7), Some(Cell::_7));
        assert_eq!(Cell::from_u8(8), Some(Cell::_8));
        assert_eq!(Cell::from_u8(9), Some(Cell::_9));
    }
}
