use crate::{board::Board, cell::Cell, group};

enum Symmetry {
    Rotate90,
    Rotate180,
    Rotate270,
    MirrorHor,
    MirrorVer,
    MirrorBoth,
}

fn generate_row_0() -> [Cell; 9] {
    [Cell::_9, Cell::_1, Cell::_8, Cell::_2, Cell::_7, Cell::_3, Cell::_6, Cell::_4, Cell::_5]
}

fn generate() -> Board {
    let row0 = [Cell::_9, Cell::_1, Cell::_8, Cell::_2, Cell::_7, Cell::_3, Cell::_6, Cell::_4, Cell::_5];
    let eewwe = [
        row0.clone(),
        [
            row0[2].clone(),
            row0[3].clone(),
            row0[4].clone(),
            row0[5].clone(),
            row0[6].clone(),
            row0[7].clone(),
            row0[8].clone(),
            row0[0].clone(),
            row0[1].clone(),
        ],
        [
            row0[2].clone(),
            row0[3].clone(),
            row0[4].clone(),
            row0[5].clone(),
            row0[6].clone(),
            row0[7].clone(),
            row0[8].clone(),
            row0[0].clone(),
            row0[1].clone(),
        ],
        [
            row0[2].clone(),
            row0[3].clone(),
            row0[4].clone(),
            row0[5].clone(),
            row0[6].clone(),
            row0[7].clone(),
            row0[8].clone(),
            row0[0].clone(),
            row0[1].clone(),
        ],
        [
            row0[2].clone(),
            row0[3].clone(),
            row0[4].clone(),
            row0[5].clone(),
            row0[6].clone(),
            row0[7].clone(),
            row0[8].clone(),
            row0[0].clone(),
            row0[1].clone(),
        ],
        [
            row0[2].clone(),
            row0[3].clone(),
            row0[4].clone(),
            row0[5].clone(),
            row0[6].clone(),
            row0[7].clone(),
            row0[8].clone(),
            row0[0].clone(),
            row0[1].clone(),
        ],
        [
            row0[2].clone(),
            row0[3].clone(),
            row0[4].clone(),
            row0[5].clone(),
            row0[6].clone(),
            row0[7].clone(),
            row0[8].clone(),
            row0[0].clone(),
            row0[1].clone(),
        ],
    ];

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
    ];
}

#[cfg(test)]
mod tests {
    use super::generate;
    use crate::{board, cell::Cell};

    #[test]
    fn test_generate() {
        assert_eq!(
            generate(),
            board::from_str([
                "918273645",
                "273645918",
                "645918273",
                "182736459",
                "736459182",
                "459182736",
                "827364519",
                "364591827",
                "591827364",
            ])
        )
    }
}
