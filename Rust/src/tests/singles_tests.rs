use crate::board::{Board, print_board};
use crate::candidates::{compute_candidates};
use crate::strategies::singles::{apply_hidden_single, apply_naked_single};


#[test]
fn test_apply_naked_single() {
    println!("testing apply_naked_single");

    let mut board: [[u8; 9]; 9] = [
        [5,3,0,0,0,0,0,0,0],
        [6,7,2,0,0,0,0,0,0],
        [1,9,8,0,0,0,0,0,0],
        [0;9],
        [0;9],
        [0;9],
        [0;9],
        [0;9],
        [0;9],
    ];
    let mut candidates = compute_candidates(&board);
    let changed =
        apply_naked_single(
            &mut board,
            &mut candidates,
        );
    assert!(changed);
    assert_eq!(board[0][2], 4);
    print_board(&board);
}



#[test]
fn test_hidden_single_row() {
    let mut board:Board = [[0u8; 9]; 9];

    let mut candidates =
        [[0u16; 9]; 9];

    candidates[0][0] =
        (1 << 0) | (1 << 1);

    candidates[0][1] =
        (1 << 0) | (1 << 1);

    candidates[0][2] =
        1 << 4;

    let mut changed = true;
    while changed{
        changed = apply_hidden_single(
            &mut board,
            &mut candidates,
        );
        println!("Hidden single: {}", changed);
    }

    assert_eq!(
        board[0][2],
        5
    );
}

#[test]
fn test_hidden_single_column() {
    let mut board = [[0u8; 9]; 9];

    let mut candidates =
        [[0u16; 9]; 9];

    candidates[0][0] =
        1 << 0;

    candidates[1][0] =
        1 << 1;

    candidates[2][0] =
        1 << 2;

    candidates[3][0] =
        1 << 4;

    let mut changed = true;
    while changed{
        changed = apply_hidden_single(
            &mut board,
            &mut candidates,
        );
        println!("Hidden single: {}", changed);
    }

    assert_eq!(
        board[3][0],
        5
    );
}

#[test]
fn test_hidden_single_box() {
    let mut board = [[0u8; 9]; 9];

    let mut candidates =
        [[0u16; 9]; 9];

    candidates[0][0] =
        (1 << 0) | (1 << 1);

    candidates[0][1] =
        (1 << 0) | (1 << 1);

    candidates[1][1] =
        1 << 4;

    let mut changed = true;
    while changed{
        changed = apply_hidden_single(
            &mut board,
            &mut candidates,
        );
        println!("Hidden single: {}", changed);
    }

    assert_eq!(
        board[1][1],
        5
    );
}