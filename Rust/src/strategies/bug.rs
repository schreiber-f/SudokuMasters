use crate::board::Board;
use crate::candidates::{
    apply_value,
    count_bits,
    Candidates,
};

pub fn apply_bug_plus_one(
    board: &mut Board,
    candidates: &mut Candidates,
) -> bool {
    let mut bug_cell = None;

    // Es darf genau eine Zelle mit 3 Kandidaten geben.
    // Alle anderen ungelösten Zellen müssen genau 2 Kandidaten besitzen.
    for row in 0..9 {
        for col in 0..9 {
            match count_bits(candidates[row][col]) {
                0 => {} // bereits gelöst
                2 => {}
                3 => {
                    if bug_cell.is_some() {
                        return false;
                    }
                    bug_cell = Some((row, col));
                }
                _ => return false,
            }
        }
    }

    let Some((row, col)) = bug_cell else {
        return false;
    };

    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;

    let mut mask = candidates[row][col];

    let mut solution = None;

    while mask != 0 {
        let digit_mask = mask & (!mask + 1);
        let digit = digit_mask.trailing_zeros() as u8 + 1;

        // ---------------- Row ----------------
        let mut row_count = 0;
        for c in 0..9 {
            if candidates[row][c] & digit_mask != 0 {
                row_count += 1;
            }
        }

        // ---------------- Column ----------------
        let mut col_count = 0;
        for r in 0..9 {
            if candidates[r][col] & digit_mask != 0 {
                col_count += 1;
            }
        }

        // ---------------- Box ----------------
        let mut box_count = 0;
        for r in start_row..start_row + 3 {
            for c in start_col..start_col + 3 {
                if candidates[r][c] & digit_mask != 0 {
                    box_count += 1;
                }
            }
        }
        println!(
            "digit={} row={} col={} box={}",
            digit,
            row_count,
            col_count,
            box_count
        );

        if row_count == 3 &&
            col_count == 3 &&
            box_count == 3
        {
            if solution.is_some() {
                // Zwei Kandidaten erfüllen BUG -> ungültig
                return false;
            }

            solution = Some(digit);
        }

        mask &= mask - 1;
    }

    if let Some(digit) = solution {
        apply_value(board, candidates, row, col, digit);
        return true;
    }

    false
}