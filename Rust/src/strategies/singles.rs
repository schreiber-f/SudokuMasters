use crate::board::{Board};
use crate::candidates::{apply_value, count_bits, single_candidate, Candidates};

pub fn apply_naked_single(
    board: &mut Board,
    candidates: &mut Candidates,
) -> bool{
    for row in 0..9 {
        for col in 0..9 {
            let val = board[row][col];
            let mask = candidates[row][col];
            if val == 0 {
                if count_bits(mask) == 1{
                    let val = single_candidate(mask).unwrap();
                    apply_value(board, candidates, row, col, val);
                    return true;
                }
            }
        }
    }
    false
}


pub fn apply_hidden_single(board: &mut Board, candidates: &mut Candidates) -> bool{

    for row in 0..9 {
        let mut counts_row = [0u8;9];
        let mut last_seen_row = [0u8;9];
        let mut counts_col = [0u8;9];
        let mut last_seen_col = [0u8;9];

        for col in 0..9 {
            let mut mask = candidates[row][col];
            let mut mask_col = candidates[col][row];

            if board[row][col] == 0 {

                while mask > 0 {
                    let trailing = mask.trailing_zeros() as usize;

                    counts_row[trailing] += 1;
                    last_seen_row[trailing] = col as u8;

                    mask &= mask - 1;
                }

            }
            if board[col][row] == 0 {

                while mask_col > 0 {
                    let trailing = mask_col.trailing_zeros() as usize;

                    counts_col[trailing] += 1;
                    last_seen_col[trailing] = col as u8;

                    mask_col &= mask_col - 1;
                }

            }
        }
        for i in 0..9 {
            let val = (i + 1) as u8;
            if counts_row[i] == 1{
                println!(
                    "ROW hidden single: val={} row={} col={}",
                    val,
                    row,
                    last_seen_row[i]
                );
                apply_value(board, candidates, row, last_seen_row[i] as usize, val);
                println!("Changed field: {}", board[row][last_seen_row[i] as usize]);

                return true;
            }
            if counts_col[i] == 1{
                println!(
                    "COL hidden single: val={} row={} col={}",
                    val,
                    last_seen_col[i],
                    row
                );
                apply_value(board, candidates, last_seen_col[i] as usize, row, val);
                println!("Changed field: {}", board[last_seen_col[i] as usize][row]);
                return true;
            }
        }
    }

    for box_idx in 0..9 {
        let mut counts_box = [0u8; 9];
        let mut last_box_row = [0usize; 9];
        let mut last_box_col = [0usize; 9];

        let start_row = (box_idx / 3) * 3;
        let start_col = (box_idx % 3) * 3;

        for j in 0..9 {
            let row = start_row + (j / 3);
            let col = start_col + (j % 3);

            if board[row][col] == 0 {
                let mut mask_box = candidates[row][col];
                while mask_box > 0 {
                    let bit = mask_box.trailing_zeros() as usize;
                    counts_box[bit] += 1;
                    last_box_row[bit] = row;
                    last_box_col[bit] = col;
                    mask_box &= mask_box - 1;
                }
            }
        }

        for bit in 0..9 {
            if counts_box[bit] == 1 {
                let row = last_box_row[bit];
                let col = last_box_col[bit];
                let val = (bit + 1) as u8;
                apply_value(board, candidates, row, col, val);
                return true;
            }
        }
    }

    false
}