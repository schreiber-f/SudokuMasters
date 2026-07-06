use crate::strategies::bug::{apply_bug_plus_one};

#[test]
fn test_bug_plus_one_real_example() {
    let mut board = [[0u8;9];9];
    let mut candidates = [[0u16;9];9];

    macro_rules! m {
        ($($x:expr),*) => {{
            let mut mask = 0u16;
            $(
                mask |= 1 << ($x - 1);
            )*
            mask
        }};
    }

    // Zeile 0
    board[0] = [2,8,9,7,4,6,5,1,3];

    // Zeile 1
    board[1] = [4,3,6,5,9,1,2,7,8];

    // Zeile 2
    board[2] = [1,7,5,3,2,8,4,9,6];

    // Zeile 3
    candidates[3][3]=m!(1,4);
    candidates[3][5]=m!(3,9);
    candidates[3][6]=m!(1,9);
    candidates[3][7]=m!(3,4);

    board[3][0]=8;
    board[3][1]=5;
    board[3][2]=7;
    board[3][4]=6;
    board[3][8]=2;

    // Zeile 4
    candidates[4][0]=m!(6,9);
    candidates[4][2]=m!(1,3);
    candidates[4][6]=m!(1,9);
    candidates[4][7]=m!(3,6);

    board[4][1]=4;
    board[4][3]=2;
    board[4][4]=8;
    board[4][5]=7;
    board[4][8]=5;

    // Zeile 5
    candidates[5][0]=m!(6,9);
    candidates[5][2]=m!(1,3);
    candidates[5][3]=m!(1,4);
    candidates[5][5]=m!(3,9);
    candidates[5][7]=m!(3,4,6); // <-- BUG+1

    board[5][1]=2;
    board[5][4]=5;
    board[5][6]=8;
    board[5][8]=7;

    // Zeile 6
    board[6]=[5,9,8,6,7,4,3,2,1];

    // Zeile 7
    board[7]=[3,6,4,8,1,2,7,5,9];

    // Zeile 8
    board[8]=[7,1,2,9,3,5,6,8,4];

    assert!(apply_bug_plus_one(
        &mut board,
        &mut candidates,
    ));

    assert_eq!(board[5][7], 3);
}

#[test]
fn test_bug_no_triple() {
    let mut board = [[0u8;9];9];
    let mut candidates = [[0u16;9];9];

    let pair = (1<<0)|(1<<1);

    for r in 0..9 {
        for c in 0..9 {
            candidates[r][c] = pair;
        }
    }

    assert!(!apply_bug_plus_one(
        &mut board,
        &mut candidates,
    ));
}

#[test]
fn test_bug_two_triples() {
    let mut board = [[0u8;9];9];
    let mut candidates = [[0u16;9];9];

    let pair = (1<<0)|(1<<1);

    for r in 0..9 {
        for c in 0..9 {
            candidates[r][c] = pair;
        }
    }

    candidates[0][0] |= 1<<2;
    candidates[1][1] |= 1<<2;

    assert!(!apply_bug_plus_one(
        &mut board,
        &mut candidates,
    ));
}