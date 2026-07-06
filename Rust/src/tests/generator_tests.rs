use crate::board::{print_board, full_board_valid};
use crate::solver::{has_unique_solution};
use crate::generator::{count_givens, generate_full_board, dig_holes};

#[test]
fn test_random_solver() {
    println!("testing random_solver");
    let board = generate_full_board();

    print_board(&board);

    assert!(full_board_valid(&board));
}

#[test]
fn test_dig_holes_target(){
    println!("testing dig_holes_target");
    let mut board =
        generate_full_board();

    dig_holes(
        &mut board,
        30,
    );
    print_board(&board);
    println!("Fields left: {}", count_givens(&board));
    assert!(has_unique_solution(&board))
}