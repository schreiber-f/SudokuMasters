use crate::board::{Board, is_valid};

pub fn find_empty(
    board: &Board,
) -> Option<(usize, usize)> {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn solve(
    board: &mut Board,
) -> bool {
    let Some((row, col)) = find_empty(board) else {
        return true;
    };

    for num in 1..=9 {
        if is_valid(board, row, col, num) {
            board[row][col] = num;

            if solve(board) {
                return true;
            }

            board[row][col] = 0;
        }
    }

    false
}

pub fn solve_random(
    board: &mut Board,
) -> bool {
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();

    let Some((row, col)) = find_empty(board) else {
        return true;
    };
    let mut nums:[u8; 9] = [1,2,3,4,5,6,7,8,9];
    nums.shuffle(&mut rng);
    for &num in nums.iter() {
        if is_valid(board, row, col, num) {
            board[row][col] = num;

            if solve_random(board) {
                return true;
            }

            board[row][col] = 0;
        }
    }

    false
}


pub fn count_solutions_recursive(board: &mut Board, count: &mut usize) {
    if *count > 1 {
        return;
    }

    let Some((row, col)) = find_empty(board) else {
        *count += 1;
        return;
    };

    for num in 1..=9 {
        if is_valid(board, row, col, num) {
            board[row][col] = num;

            count_solutions_recursive(
                board,
                count,
            );

            board[row][col] = 0;
        }
    }
}


pub fn count_solutions(board: &Board) -> usize{
    let mut board_copy = *board;
    let mut count = 0;

    count_solutions_recursive(
        &mut board_copy,
        &mut count,
    );

    //println!("We have {} solutions\n", count);

    return count;
}


pub fn has_unique_solution(
    board: &Board,
) -> bool {
    count_solutions(board) == 1
}