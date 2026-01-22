use crate::{
    board::{Board, SolvedBoard},
    cell::Cell,
    solved_group::SolvedGroup,
};

enum Symmetry {
    Rotate90,
    Rotate180,
    Rotate270,
    MirrorHor,
    MirrorVer,
    MirrorBoth,
}

fn generate_row_0() -> SolvedGroup {
    [Cell::_9, Cell::_1, Cell::_8, Cell::_2, Cell::_7, Cell::_3, Cell::_6, Cell::_4, Cell::_5]
}

fn generate_board_latin_square(row_0: &[Cell; 9]) -> SolvedBoard {
    [
        row_0.clone(),
        [row_0[3], row_0[4], row_0[5], row_0[6], row_0[7], row_0[8], row_0[0], row_0[1], row_0[2]],
        [row_0[6], row_0[7], row_0[8], row_0[0], row_0[1], row_0[2], row_0[3], row_0[4], row_0[5]],
        [row_0[1], row_0[2], row_0[3], row_0[4], row_0[5], row_0[6], row_0[7], row_0[8], row_0[0]],
        [row_0[4], row_0[5], row_0[6], row_0[7], row_0[8], row_0[0], row_0[1], row_0[2], row_0[3]],
        [row_0[7], row_0[8], row_0[0], row_0[1], row_0[2], row_0[3], row_0[4], row_0[5], row_0[6]],
        [row_0[2], row_0[3], row_0[4], row_0[5], row_0[6], row_0[7], row_0[8], row_0[0], row_0[1]],
        [row_0[5], row_0[6], row_0[7], row_0[8], row_0[0], row_0[1], row_0[2], row_0[3], row_0[4]],
        [row_0[8], row_0[0], row_0[1], row_0[2], row_0[3], row_0[4], row_0[5], row_0[6], row_0[7]],
    ]
}

fn rotate_90() {}
fn rotate_180() {}
fn rotate_270() {}

fn mirror_hor() {}
fn mirror_ver() {}
fn mirror_both() {}

fn generate() -> Board {
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
    use super::{generate, generate_board_latin_square, generate_row_0};
    use crate::{board, cell::Cell, group};

    //#[test]
    //fn test_generate() {
    //    assert_eq!(
    //        board::to_string(&generate()),
    //        board::to_string(&board::from_str([
    //            "918273645",
    //            "273645918",
    //            "645918273",
    //            "182736459",
    //            "736459182",
    //            "459182736",
    //            "827364519",
    //            "364591827",
    //            "591827364",
    //        ]))
    //    )
    //}

    #[test]
    fn test_generate_row_0() {
        assert_eq!(
            generate_row_0(),
            [
                Cell::_9,
                Cell::_1,
                Cell::_8,
                Cell::_2,
                Cell::_7,
                Cell::_3,
                Cell::_6,
                Cell::_4,
                Cell::_5
            ]
        )
    }

    #[test]
    fn test_generate_board_latin_square() {
        assert_eq!(
            generate_board_latin_square(&[
                Cell::_9,
                Cell::_1,
                Cell::_8,
                Cell::_2,
                Cell::_7,
                Cell::_3,
                Cell::_6,
                Cell::_4,
                Cell::_5
            ]),
            [
                [
                    Cell::_9,
                    Cell::_1,
                    Cell::_8,
                    Cell::_2,
                    Cell::_7,
                    Cell::_3,
                    Cell::_6,
                    Cell::_4,
                    Cell::_5
                ],
                [
                    Cell::_2,
                    Cell::_7,
                    Cell::_3,
                    Cell::_6,
                    Cell::_4,
                    Cell::_5,
                    Cell::_9,
                    Cell::_1,
                    Cell::_8
                ],
                [
                    Cell::_6,
                    Cell::_4,
                    Cell::_5,
                    Cell::_9,
                    Cell::_1,
                    Cell::_8,
                    Cell::_2,
                    Cell::_7,
                    Cell::_3
                ],
                [
                    Cell::_1,
                    Cell::_8,
                    Cell::_2,
                    Cell::_7,
                    Cell::_3,
                    Cell::_6,
                    Cell::_4,
                    Cell::_5,
                    Cell::_9
                ],
                [
                    Cell::_7,
                    Cell::_3,
                    Cell::_6,
                    Cell::_4,
                    Cell::_5,
                    Cell::_9,
                    Cell::_1,
                    Cell::_8,
                    Cell::_2
                ],
                [
                    Cell::_4,
                    Cell::_5,
                    Cell::_9,
                    Cell::_1,
                    Cell::_8,
                    Cell::_2,
                    Cell::_7,
                    Cell::_3,
                    Cell::_6
                ],
                [
                    Cell::_8,
                    Cell::_2,
                    Cell::_7,
                    Cell::_3,
                    Cell::_6,
                    Cell::_4,
                    Cell::_5,
                    Cell::_9,
                    Cell::_1
                ],
                [
                    Cell::_3,
                    Cell::_6,
                    Cell::_4,
                    Cell::_5,
                    Cell::_9,
                    Cell::_1,
                    Cell::_8,
                    Cell::_2,
                    Cell::_7
                ],
                [
                    Cell::_5,
                    Cell::_9,
                    Cell::_1,
                    Cell::_8,
                    Cell::_2,
                    Cell::_7,
                    Cell::_3,
                    Cell::_6,
                    Cell::_4
                ],
            ]
        )
    }
}

// alterna linhas do mesmo trio
// alterna colunas do mesmo trio

// alterna trios de linhas
// alterna trios de colunas
