use crate::board::{print_board, full_board_valid};
use crate::solver::{find_empty, solve, count_solutions};
use crate::generator::{generate_full_board, dig_holes};
use crate::human_solver::human_solve;

#[test]
fn test_find_empty_none() {
    println!("testing find_empty_none");
    let board = [
        [1,2,3,4,5,6,7,8,9],
        [4,5,6,7,8,9,1,2,3],
        [7,8,9,1,2,3,4,5,6],

        [2,3,4,5,6,7,8,9,1],
        [5,6,7,8,9,1,2,3,4],
        [8,9,1,2,3,4,5,6,7],

        [3,4,5,6,7,8,9,1,2],
        [6,7,8,9,1,2,3,4,5],
        [9,1,2,3,4,5,6,7,8],
    ];

    assert_eq!(find_empty(&board), None);
}

#[test]
fn test_solver_completes_board() {
    println!("testing solver_completes_board");
    let mut board = [
        [5,3,0,0,7,0,0,0,0],
        [6,0,0,1,9,5,0,0,0],
        [0,9,8,0,0,0,0,6,0],

        [8,0,0,0,6,0,0,0,3],
        [4,0,0,8,0,3,0,0,1],
        [7,0,0,0,2,0,0,0,6],

        [0,6,0,0,0,0,2,8,0],
        [0,0,0,4,1,9,0,0,5],
        [0,0,0,0,8,0,0,7,9],
    ];

    solve(&mut board);

    print_board(&board);
    assert!(full_board_valid(&board));
}

#[test] fn test_count_solutions_full_board() {
    println!("testing count_solutions_full_board");
    let board = generate_full_board();
    print_board(&board);
    assert_eq!( count_solutions(&board), 1 );
}

#[test]
fn test_count_solutions_empty_board() {
    println!("testing count_solutions_empty_board");
    let board = [[0u8; 9]; 9];
    print_board(&board);
    assert!( count_solutions(&board) > 1 );
}

#[test]
fn test_human_solver_singles() {
    println!("testing human_solver_singles");
    let mut board = generate_full_board();
    println!("board before:");
    print_board(&board);
    dig_holes(&mut board, 35);

    println!("board with holes:");
    print_board(&board);

    let report = human_solve(&mut board);

    assert!(report.is_solved);
    println!("board after:");
    print_board(&board);
    println!("solve report {:?}", report);
}

#[test]
fn test_human_solver_pairs() {
    println!("testing human_solver_singles_and_pairs...");
    let mut board = [[1, 0, 0, 0, 0, 3, 2, 4, 6],
        [3, 6, 0, 4, 5, 0, 0, 1, 0],
        [2, 0, 0, 0, 1, 0, 3, 5, 0],
        [0, 0, 5, 7, 0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0, 9, 0, 0, 0],
        [0, 0, 6, 0, 8, 0, 0, 0, 9],
        [5, 0, 4, 1, 0, 0, 0, 0, 0],
        [7, 0, 1, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 9, 0, 0, 0, 0]];
    println!("board before:");
    print_board(&board);

    let report = human_solve(&mut board);

    assert!(report.is_solved);
    println!("board after:");
    print_board(&board);
    println!("solve report {:?}", report);
}