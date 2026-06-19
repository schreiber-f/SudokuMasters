use crate::candidates::{Candidates, count_bits, remove_candidates};
use crate::strategies::cell_utils::{box_cells};


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