use crate::{board::Board, cell::Cell, solved_board::SolvedBoard, solved_group::SolvedGroup};

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
        *row_0,
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

fn offset() {}

fn rotate_90() {}
fn rotate_180() {}
fn rotate_270() {}

fn mirror_hor() {}
fn mirror_ver() {}
fn mirror_both() {}

fn shake_group_rows() {}
fn shake_group_cols() {}

fn shake_board_group_rows() {}
fn shake_board_group_cols() {}

fn generate() -> Board {
    [
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

#[cfg(test)]
mod tests {
    use crate::{cell::Cell, solved_board::from_str};
    use super::{generate_board_latin_square, generate_row_0};

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
            from_str([
                "918273645",
                "273645918",
                "645918273",
                "182736459",
                "736459182",
                "459182736",
                "827364591",
                "364591827",
                "591827364",
            ])
        )
    }
}
