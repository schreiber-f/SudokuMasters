use crate::candidates::{Candidates, count_bits, remove_candidates};
use crate::strategies::cell_utils::{row_cells, col_cells, box_cells};
use crate::board::{box_index};

pub fn find_pointing_pair_in_box(
    candidates: &mut Candidates,
    box_idx: usize,
) -> bool{
    let mut changed = false;
    let cells = box_cells(box_idx);
    let start_row = (box_idx / 3) * 3;
    let start_col = (box_idx % 3) * 3;

    for digit in 1..=9{
        let mut row_mask = 0u16;
        let mut col_mask = 0u16;
        let digit_mask = 1 << (digit - 1);
        let mut positions:Vec<(usize, usize)> = Vec::new();
        for (r, c) in cells {
            let mask = candidates[r][c];
            if (mask & digit_mask) != 0 {
                row_mask |= 1 << r;
                col_mask |= 1 << c;
                positions.push((r, c));
            }
        }
        if positions.len() >= 2 && count_bits(row_mask) == 1{
            let t_r= positions[0].0;
            for col in 0..9{
                if col >= start_col &&
                    col < start_col + 3
                {
                    continue;
                }
                let mask = candidates[t_r][col];
                if (mask & digit_mask) != 0 {
                    if remove_candidates(candidates, t_r, col, digit_mask) {
                        changed = true;
                    }
                }
            }
        }

        if positions.len() >= 2 && count_bits(col_mask) == 1{
            let t_c = positions[0].1;
            for row in 0..9{
                if row >= start_row && row < start_row + 3 {
                    continue;
                }
                let mask = candidates[row][t_c];
                if (mask & digit_mask) != 0 {
                    if remove_candidates(candidates, row, t_c, digit_mask) {
                        changed = true;
                    }
                }
            }
        }
        if changed{
            return true;
        }
    }

    changed
}

pub fn apply_pointing_pair(
    candidates: &mut Candidates,
) -> bool {

    for box_idx in 0..9 {
        if find_pointing_pair_in_box(
            candidates,
            box_idx,
        ) {
            return true;
        }
    }

    false
}


pub fn find_claiming_in_unit(
candidates: &mut Candidates,
unit_cells: &[(usize, usize)],
) -> bool {
    let mut changed = false;

    for digit in 1..=9{
        let digit_mask:u16 = 1 << (digit - 1);
        let mut positions:Vec<(usize, usize)> = Vec::new();

        for &(r, c) in unit_cells {
            let mask = candidates[r][c];
            if (mask & digit_mask) != 0 {
                positions.push((r,c));
            }
        }

        if positions.len() < 2 {
            continue;
        }

        let first_box = box_index(positions[0].0, positions[0].1);

        if !positions.iter().all(|&(r,c)| {
            box_index(r,c) == first_box
        }) {
            continue;
        }

        for (r,c) in box_cells(first_box) {

            if positions.contains(&(r,c)) {
                continue;
            }

            if remove_candidates(
                candidates,
                r,
                c,
                digit_mask,
            ) {
                changed = true;
            }
        }
        if changed {
            return true;
        }
    }
    changed
}


pub fn apply_box_line_reduction(
    candidates: &mut Candidates,
) -> bool {

    for row in 0..9 {
        let cells = row_cells(row);

        if find_claiming_in_unit(
            candidates,
            &cells,
        ) {
            return true;
        }
    }

    for col in 0..9 {
        let cells = col_cells(col);

        if find_claiming_in_unit(
            candidates,
            &cells,
        ) {
            return true;
        }
    }

    false
}