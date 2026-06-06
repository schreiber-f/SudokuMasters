use crate::solver::{has_unique_solution, solve_random};
use crate::board::Board;
pub fn generate_full_board()-> Board {
    let mut board = Board::default();
    solve_random(&mut board);
    board
}


pub fn count_givens(board: &Board) -> usize{
    let mut count = 0;
    for row in board.iter(){
        for cell in row.iter(){
            if *cell != 0 {
                count += 1;
            }
        }
    }

    count
}


pub fn dig_holes(
    board: &mut Board,
    target_givens: usize,
){
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();

    let mut filled_fields: Vec<(usize, usize)> = Vec::new();
    for i in 0..9{
        for j in 0..9{
            if board[i][j] != 0 {
                filled_fields.push((i, j));
            }
        }
    }

    let mut curr_filled_fields = count_givens(board);

    while curr_filled_fields > target_givens && !filled_fields.is_empty(){
        filled_fields.shuffle(&mut rng);
        let (row, col) = filled_fields.pop().unwrap();
        let old = board[row][col];
        board[row][col] = 0;
        if has_unique_solution(board) {
            curr_filled_fields -= 1
        } else {
            board[row][col] = old;
        }
    }
}