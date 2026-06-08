use crate::board::Board;

pub type Candidates = [[u16; 9]; 9];
const ALL_CANDIDATES: u16 = 0x1FF;
const SOLVED: u16 = 0;

pub fn compute_candidates(board: &Board) -> Candidates {
    let mut candidates = [[ALL_CANDIDATES; 9]; 9];

    for row in 0..9 {
        for col in 0..9 {
            let val = board[row][col];

            if val > 0{

                update_candidates_after_placement(&mut candidates, row, col, val)

            }
        }
    }
    candidates
}


pub fn update_candidates_after_placement(candidates: &mut Candidates, row:usize, col:usize, val:u8){
    candidates[row][col] = SOLVED;

    let remove_mask = !(1 << (val - 1));

    for i in 0..9 {
        if i != col { candidates[row][i] &= remove_mask; }
        if i != row { candidates[i][col] &= remove_mask; }
    }

    let box_row_start = (row / 3) * 3;
    let box_col_start = (col / 3) * 3;

    for r in box_row_start..box_row_start + 3 {
        for c in box_col_start..box_col_start + 3 {
            if r != row || c != col {
                candidates[r][c] &= remove_mask;
            }
        }
    }
}


pub fn apply_value(board: &mut Board, candidates: &mut Candidates, row:usize, col:usize, val:u8) {
    if board[row][col] != 0 {
        return;
    }

    update_candidates_after_placement(candidates, row, col, val);

    board[row][col] = val;
}

pub fn count_bits(mask: u16) -> u32{
    mask.count_ones()
}

pub fn single_candidate(mask: u16) -> Option<u8> {
    let bits_count = count_bits(mask);
    if bits_count == 1 {
        let val = mask.trailing_zeros() + 1;
        Some(val as u8)
    }else {
        None
    }
}