
pub fn row_cells(row: usize) -> [(usize, usize); 9] {
    let mut cells = [(0, 0); 9];

    for col in 0..9{
        cells[col] = (row, col);
    }

    cells
}

pub fn col_cells(col: usize) -> [(usize, usize); 9] {
    let mut cells = [(0, 0); 9];

    for row in 0..9{
        cells[row] = (row, col);
    }

    cells
}


pub fn box_cells(box_idx: usize) -> [(usize, usize); 9] {
    let mut cells = [(0, 0); 9];

    let start_row = (box_idx / 3) * 3;
    let start_col = (box_idx % 3) * 3;

    let mut idx = 0;

    for r in start_row..start_row + 3 {
        for c in start_col..start_col + 3 {
            cells[idx] = (r, c);
            idx += 1;
        }
    }

    cells
}