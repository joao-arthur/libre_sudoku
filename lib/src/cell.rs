#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn try_from_u8(value: u8) -> Option<Cell> {
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

    pub fn from_u8(value: u8) -> Cell {
        Cell::try_from_u8(value).unwrap()
    }

    pub fn to_u8(&self) -> u8 {
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

    pub fn try_from_usize(value: usize) -> Option<Cell> {
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

    pub fn from_usize(value: usize) -> Cell {
        Cell::try_from_usize(value).unwrap()
    }

    pub fn to_usize(&self) -> usize {
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

    pub fn try_from_char(c: &char) -> Option<Cell> {
        match c {
            '1' => Some(Cell::_1),
            '2' => Some(Cell::_2),
            '3' => Some(Cell::_3),
            '4' => Some(Cell::_4),
            '5' => Some(Cell::_5),
            '6' => Some(Cell::_6),
            '7' => Some(Cell::_7),
            '8' => Some(Cell::_8),
            '9' => Some(Cell::_9),
            _ => None,
        }
    }

    pub fn from_char(c: &char) -> Cell {
        Self::try_from_char(c).unwrap()
    }

    pub fn to_char(self: &Cell) -> char {
        match self {
            Cell::_1 => '1',
            Cell::_2 => '2',
            Cell::_3 => '3',
            Cell::_4 => '4',
            Cell::_5 => '5',
            Cell::_6 => '6',
            Cell::_7 => '7',
            Cell::_8 => '8',
            Cell::_9 => '9',
        }
    }

    pub fn try_from_str(s: &str) -> Option<Cell> {
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

    pub fn from_str(s: &str) -> Cell {
        Cell::try_from_str(s).unwrap()
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
mod tests {
    use super::Cell;

    #[test]
    fn try_from_u8() {
        assert_eq!(Cell::try_from_u8(0), Some(Cell::_1));
        assert_eq!(Cell::try_from_u8(1), Some(Cell::_2));
        assert_eq!(Cell::try_from_u8(2), Some(Cell::_3));
        assert_eq!(Cell::try_from_u8(3), Some(Cell::_4));
        assert_eq!(Cell::try_from_u8(4), Some(Cell::_5));
        assert_eq!(Cell::try_from_u8(5), Some(Cell::_6));
        assert_eq!(Cell::try_from_u8(6), Some(Cell::_7));
        assert_eq!(Cell::try_from_u8(7), Some(Cell::_8));
        assert_eq!(Cell::try_from_u8(8), Some(Cell::_9));
        assert_eq!(Cell::try_from_u8(9), None);
    }

    #[test]
    fn from_u8() {
        assert_eq!(Cell::from_u8(0), Cell::_1);
        assert_eq!(Cell::from_u8(1), Cell::_2);
        assert_eq!(Cell::from_u8(2), Cell::_3);
        assert_eq!(Cell::from_u8(3), Cell::_4);
        assert_eq!(Cell::from_u8(4), Cell::_5);
        assert_eq!(Cell::from_u8(5), Cell::_6);
        assert_eq!(Cell::from_u8(6), Cell::_7);
        assert_eq!(Cell::from_u8(7), Cell::_8);
        assert_eq!(Cell::from_u8(8), Cell::_9);
    }

    #[test]
    fn to_u8() {
        assert_eq!(Cell::_1.to_u8(), 0);
        assert_eq!(Cell::_2.to_u8(), 1);
        assert_eq!(Cell::_3.to_u8(), 2);
        assert_eq!(Cell::_4.to_u8(), 3);
        assert_eq!(Cell::_5.to_u8(), 4);
        assert_eq!(Cell::_6.to_u8(), 5);
        assert_eq!(Cell::_7.to_u8(), 6);
        assert_eq!(Cell::_8.to_u8(), 7);
        assert_eq!(Cell::_9.to_u8(), 8);
    }

    #[test]
    fn try_from_usize() {
        assert_eq!(Cell::try_from_usize(0), Some(Cell::_1));
        assert_eq!(Cell::try_from_usize(1), Some(Cell::_2));
        assert_eq!(Cell::try_from_usize(2), Some(Cell::_3));
        assert_eq!(Cell::try_from_usize(3), Some(Cell::_4));
        assert_eq!(Cell::try_from_usize(4), Some(Cell::_5));
        assert_eq!(Cell::try_from_usize(5), Some(Cell::_6));
        assert_eq!(Cell::try_from_usize(6), Some(Cell::_7));
        assert_eq!(Cell::try_from_usize(7), Some(Cell::_8));
        assert_eq!(Cell::try_from_usize(8), Some(Cell::_9));
        assert_eq!(Cell::try_from_usize(9), None);
    }

    #[test]
    fn from_usize() {
        assert_eq!(Cell::from_usize(0), Cell::_1);
        assert_eq!(Cell::from_usize(1), Cell::_2);
        assert_eq!(Cell::from_usize(2), Cell::_3);
        assert_eq!(Cell::from_usize(3), Cell::_4);
        assert_eq!(Cell::from_usize(4), Cell::_5);
        assert_eq!(Cell::from_usize(5), Cell::_6);
        assert_eq!(Cell::from_usize(6), Cell::_7);
        assert_eq!(Cell::from_usize(7), Cell::_8);
        assert_eq!(Cell::from_usize(8), Cell::_9);
    }

    #[test]
    fn to_usize() {
        assert_eq!(Cell::_1.to_usize(), 0);
        assert_eq!(Cell::_2.to_usize(), 1);
        assert_eq!(Cell::_3.to_usize(), 2);
        assert_eq!(Cell::_4.to_usize(), 3);
        assert_eq!(Cell::_5.to_usize(), 4);
        assert_eq!(Cell::_6.to_usize(), 5);
        assert_eq!(Cell::_7.to_usize(), 6);
        assert_eq!(Cell::_8.to_usize(), 7);
        assert_eq!(Cell::_9.to_usize(), 8);
    }

    #[test]
    fn try_from_char() {
        assert_eq!(Cell::try_from_char(&'a'), None);
        assert_eq!(Cell::try_from_char(&'0'), None);
        assert_eq!(Cell::try_from_char(&'1'), Some(Cell::_1));
        assert_eq!(Cell::try_from_char(&'2'), Some(Cell::_2));
        assert_eq!(Cell::try_from_char(&'3'), Some(Cell::_3));
        assert_eq!(Cell::try_from_char(&'4'), Some(Cell::_4));
        assert_eq!(Cell::try_from_char(&'5'), Some(Cell::_5));
        assert_eq!(Cell::try_from_char(&'6'), Some(Cell::_6));
        assert_eq!(Cell::try_from_char(&'7'), Some(Cell::_7));
        assert_eq!(Cell::try_from_char(&'8'), Some(Cell::_8));
        assert_eq!(Cell::try_from_char(&'9'), Some(Cell::_9));
    }

    #[test]
    fn from_char() {
        assert_eq!(Cell::from_char(&'1'), Cell::_1);
        assert_eq!(Cell::from_char(&'2'), Cell::_2);
        assert_eq!(Cell::from_char(&'3'), Cell::_3);
        assert_eq!(Cell::from_char(&'4'), Cell::_4);
        assert_eq!(Cell::from_char(&'5'), Cell::_5);
        assert_eq!(Cell::from_char(&'6'), Cell::_6);
        assert_eq!(Cell::from_char(&'7'), Cell::_7);
        assert_eq!(Cell::from_char(&'8'), Cell::_8);
        assert_eq!(Cell::from_char(&'9'), Cell::_9);
    }

    #[test]
    fn to_char() {
        assert_eq!(Cell::_1.to_char(), '1');
        assert_eq!(Cell::_2.to_char(), '2');
        assert_eq!(Cell::_3.to_char(), '3');
        assert_eq!(Cell::_4.to_char(), '4');
        assert_eq!(Cell::_5.to_char(), '5');
        assert_eq!(Cell::_6.to_char(), '6');
        assert_eq!(Cell::_7.to_char(), '7');
        assert_eq!(Cell::_8.to_char(), '8');
        assert_eq!(Cell::_9.to_char(), '9');
    }

    #[test]
    fn try_from_str() {
        assert_eq!(Cell::try_from_str("a"), None);
        assert_eq!(Cell::try_from_str("0"), None);
        assert_eq!(Cell::try_from_str("1"), Some(Cell::_1));
        assert_eq!(Cell::try_from_str("2"), Some(Cell::_2));
        assert_eq!(Cell::try_from_str("3"), Some(Cell::_3));
        assert_eq!(Cell::try_from_str("4"), Some(Cell::_4));
        assert_eq!(Cell::try_from_str("5"), Some(Cell::_5));
        assert_eq!(Cell::try_from_str("6"), Some(Cell::_6));
        assert_eq!(Cell::try_from_str("7"), Some(Cell::_7));
        assert_eq!(Cell::try_from_str("8"), Some(Cell::_8));
        assert_eq!(Cell::try_from_str("9"), Some(Cell::_9));
    }

    #[test]
    fn from_str() {
        assert_eq!(Cell::from_str("1"), Cell::_1);
        assert_eq!(Cell::from_str("2"), Cell::_2);
        assert_eq!(Cell::from_str("3"), Cell::_3);
        assert_eq!(Cell::from_str("4"), Cell::_4);
        assert_eq!(Cell::from_str("5"), Cell::_5);
        assert_eq!(Cell::from_str("6"), Cell::_6);
        assert_eq!(Cell::from_str("7"), Cell::_7);
        assert_eq!(Cell::from_str("8"), Cell::_8);
        assert_eq!(Cell::from_str("9"), Cell::_9);
    }

    #[test]
    fn to_str() {
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
