use crate::candidates::{count_bits, single_candidate, compute_candidates, apply_value};

#[test]
fn test_count_bits() {
    println!("testing count_bits");
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(1), 1);
    assert_eq!(count_bits(3), 2);
    assert_eq!(count_bits(0x1FF), 9);
}

#[test]
fn test_single_candidate() {
    println!("testing single_candidate");
    assert_eq!(single_candidate(1), Some(1));
    assert_eq!(single_candidate(2), Some(2));
    assert_eq!(single_candidate(8), Some(4));
    assert_eq!(single_candidate(3), None);
}

#[test]
fn test_compute_candidates_empty_board() {
    println!("testing compute_candidates_empty_board");
    let board = [[0u8; 9]; 9];

    let candidates = compute_candidates(&board);
    const ALL_CANDIDATES: u16 = 0x1FF;

    for row in 0..9 {
        for col in 0..9 {
            assert_eq!(
                candidates[row][col],
                ALL_CANDIDATES
            );
        }
    }
}

#[test]
fn test_compute_candidates_single_value() {
    println!("testing compute_candidates_single_value");
    let mut board = [[0u8; 9]; 9];

    board[0][0] = 5;

    let candidates =
        compute_candidates(&board);

    assert_eq!(
        candidates[0][0],
        0
    );

    let five_mask = 1 << 4;

    assert_eq!(
        candidates[0][1] & five_mask,
        0
    );

    assert_eq!(
        candidates[1][0] & five_mask,
        0
    );

    assert_eq!(
        candidates[1][1] & five_mask,
        0
    );
}

#[test]
fn test_apply_value() {
    let mut board = [[0u8; 9]; 9];

    let mut candidates =
        compute_candidates(&board);

    apply_value(
        &mut board,
        &mut candidates,
        0,
        0,
        5,
    );

    assert_eq!(
        board[0][0],
        5
    );

    assert_eq!(
        candidates[0][0],
        0
    );
}

#[test]
fn test_apply_value_filled_cell() {
    let mut board = [[0u8; 9]; 9];

    board[0][0] = 5;

    let mut candidates =
        compute_candidates(&board);

    apply_value(
        &mut board,
        &mut candidates,
        0,
        0,
        3,
    );

    assert_eq!(
        board[0][0],
        5
    );
}