use crate::candidates::{count_bits, remove_candidates, Candidates};
use crate::strategies::cell_utils::{box_cells, col_cells, row_cells};
use crate::strategies::pairs::{find_hidden_pairs_in_unit, find_naked_pairs_in_unit_fast};

pub fn find_naked_triple_in_unit(candidates: &mut Candidates, unit_cells: &[(usize, usize)]) -> bool {
    let mut changed = false;
    let len = unit_cells.len();

    // 1. Suche das erste Element des Paares über den Array-Index
    for i in 0..(len - 2) {
        for j in i+1..(len - 1) {
            for k in j+1..len {
                let mask1 = candidates[unit_cells[i].0][unit_cells[i].1];
                let mask2 = candidates[unit_cells[j].0][unit_cells[j].1];
                let mask3 = candidates[unit_cells[k].0][unit_cells[k].1];

                let union = mask1 | mask2 | mask3;

                if count_bits(mask1) == 0
                    || count_bits(mask1) > 3
                {
                    continue;
                }

                if count_bits(mask2) == 0
                    || count_bits(mask2) > 3
                {
                    continue;
                }

                if count_bits(mask3) == 0
                    || count_bits(mask3) > 3
                {
                    continue;
                }

                let mut subset_count = 0;

                for &(r,c) in unit_cells {

                    let mask = candidates[r][c];

                    if mask != 0 && (mask | union) == union {
                        subset_count += 1;
                    }
                }

                if subset_count != 3 {
                    continue;
                }

                if count_bits(union)==3{
                    for c in 0..len {
                        if c == i || c == j || c == k {
                            continue; // Überspringe die beiden Pair-Zellen
                        }

                        let (r_other, c_other) = unit_cells[c];

                        if (candidates[r_other][c_other] & union) != 0 {
                            changed |= remove_candidates(candidates, r_other, c_other, union);
                        }
                    }
                }
                if changed {
                    return true;
                }
            }
        }
    }

    changed
}



pub fn find_hidden_triple_in_unit(
    candidates: &mut Candidates,
    unit_cells: &[(usize, usize)]
) -> bool {
    let mut changed = false;

    let mut digit_positions = [0u16; 9];

    // Positionen jeder Zahl sammeln
    for cell_idx in 0..9 {
        let (r, c) = unit_cells[cell_idx];
        let mask = candidates[r][c];

        for digit in 0..9 {
            if (mask & (1 << digit)) != 0 {
                digit_positions[digit] |= 1 << cell_idx;
            }
        }
    }

    for d1 in 0..7 {

        let pos1 = digit_positions[d1];

        if count_bits(pos1) < 2 || count_bits(pos1) > 3 {
            continue;
        }

        for d2 in (d1 + 1)..8 {

            let pos2 = digit_positions[d2];

            if count_bits(pos2) < 2 || count_bits(pos2) > 3 {
                continue;
            }

            for d3 in (d2 + 1)..9 {

                let pos3 = digit_positions[d3];

                if count_bits(pos3) < 2 || count_bits(pos3) > 3 {
                    continue;
                }

                let union = pos1 | pos2 | pos3;

                if count_bits(union) != 3 {
                    continue;
                }

                let triple_mask =
                    (1 << d1) |
                        (1 << d2) |
                        (1 << d3);

                for cell_idx in 0..9 {

                    if (union & (1 << cell_idx)) == 0 {
                        continue;
                    }

                    let (r, c) = unit_cells[cell_idx];

                    let to_remove =
                        candidates[r][c] & !triple_mask;

                    if to_remove != 0 {
                        changed |= remove_candidates(
                            candidates,
                            r,
                            c,
                            to_remove,
                        );
                    }
                }

                if changed {
                    return true;
                }
            }
        }
    }

    changed
}


pub fn apply_naked_triple(
    candidates: &mut Candidates,
)->bool{
    for row in 0..9 {
        let cells = row_cells(row);

        if find_naked_triple_in_unit(candidates, &cells) {
            return true;
        }
    }

    for col in 0..9 {
        let cells = col_cells(col);

        if find_naked_triple_in_unit(candidates, &cells) {
            return true;
        }
    }

    for box_idx in 0..9 {
        let cells = box_cells(box_idx);

        if find_naked_triple_in_unit(candidates, &cells) {
            return true;
        }
    }

    false
}

pub fn apply_hidden_triple(
    candidates: &mut Candidates,
) -> bool {

    for row in 0..9 {
        let cells = row_cells(row);

        if find_hidden_triple_in_unit(
            candidates,
            &cells
        ) {
            return true;
        }
    }

    for col in 0..9 {
        let cells = col_cells(col);

        if find_hidden_triple_in_unit(
            candidates,
            &cells
        ) {
            return true;
        }
    }

    for box_idx in 0..9 {
        let cells = box_cells(box_idx);

        if find_hidden_triple_in_unit(
            candidates,
            &cells
        ) {
            return true;
        }
    }

    false
}