use crate::candidates::{Candidates, count_bits, remove_candidates, build_row_mask, build_col_mask};

pub fn find_x_wing_rows(candidates: &mut Candidates) -> bool {
    let mut changed = false;

    for digit in 1..=9 {
        let digit_mask = 1 << (digit - 1);

        // Für jede Zeile: In welchen Spalten kommt die Zahl vor?
        let mut row_masks = [0u16; 9];

        for row in 0..9 {
            row_masks[row] = build_row_mask(candidates, row, digit_mask);
        }

        // Zwei Zeilen mit identischer 2-Bit-Maske suchen
        for r1 in 0..8 {
            if count_bits(row_masks[r1]) != 2 {
                continue;
            }

            for r2 in (r1 + 1)..9 {
                if row_masks[r1] != row_masks[r2] {
                    continue;
                }

                // Aus allen anderen Zeilen löschen
                for col in 0..9 {
                    if row_masks[r1] & (1 << col) == 0 {
                        continue;
                    }

                    for row in 0..9 {
                        if row == r1 || row == r2 {
                            continue;
                        }

                        changed |= remove_candidates(
                            candidates,
                            row,
                            col,
                            digit_mask,
                        );
                    }
                }

                if changed {
                    return true;
                }
            }
        }
    }

    false
}

pub fn find_x_wing_cols(candidates: &mut Candidates) -> bool {
    let mut changed = false;

    for digit in 1..=9 {
        let digit_mask = 1 << (digit - 1);

        let mut col_masks = [0u16; 9];

        for col in 0..9 {
            col_masks[col] = build_col_mask(candidates, col, digit_mask);
        }

        for c1 in 0..8 {
            if count_bits(col_masks[c1]) != 2 {
                continue;
            }

            for c2 in (c1 + 1)..9 {
                if col_masks[c1] != col_masks[c2] {
                    continue;
                }

                for row in 0..9 {

                    if col_masks[c1] & (1 << row) == 0 {
                        continue;
                    }

                    for col in 0..9 {

                        if col == c1 || col == c2 {
                            continue;
                        }

                        changed |= remove_candidates(
                            candidates,
                            row,
                            col,
                            digit_mask,
                        );
                    }
                }

                if changed {
                    return true;
                }
            }
        }
    }

    false
}

pub fn apply_x_wing(
    candidates: &mut Candidates,
) -> bool {

    if find_x_wing_rows(candidates) {
        return true;
    }

    if find_x_wing_cols(candidates) {
        return true;
    }

    false
}