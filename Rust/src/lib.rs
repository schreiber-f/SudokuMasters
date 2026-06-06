pub mod board;
pub mod solver;
pub mod generator;


#[cfg(test)]
mod tests {
    use crate::board::{is_valid, Board, print_board, full_board_valid};
    use crate::solver::{find_empty, solve, count_solutions, has_unique_solution};
    use crate::generator::{count_givens, generate_full_board, dig_holes};
    use super::*;

    #[test]
    fn test_invalid_row() {
        let board = [
            [5,3,0,0,0,0,0,0,0],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
        ];

        assert!(!is_valid(&board, 0, 2, 5));
    }
    #[test]
    fn test_invalid_column() {
        let board = [
            [5,2,0,0,0,0,0,0,0],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
        ];

        assert!(!is_valid(&board, 1, 0, 5));
    }
    #[test]
    fn test_invalid_block() {
        let board = [
            [5,0,0,0,0,0,0,0,0],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
        ];

        assert!(!is_valid(&board, 1, 1, 5));
    }
    #[test]
    fn test_valid_move() {
        let board = [
            [5,3,0,0,0,0,0,0,0],
            [6,0,0,0,0,0,0,0,0],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
        ];

        assert!(is_valid(&board, 8, 8, 1));
    }
    #[test]
    fn test_find_empty() {
        let board = [
            [5,3,1,0,0,0,0,0,0],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
            [0;9],
        ];

        assert_eq!(
            find_empty(&board),
            Some((0,3))
        );
    }
    #[test]
    fn test_find_empty_none() {
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

    #[test]
    fn test_random_solver() {
        let board = generate_full_board();

        print_board(&board);

        assert!(full_board_valid(&board));
    }

    #[test]
    fn test_generate_multiple_boards() {
        for _ in 0..100 {
            let board = generate_full_board();

            assert!(full_board_valid(&board));
        }
    }

    #[test]
    fn test_count_solutions_full_board() {
        let board = generate_full_board();
        print_board(&board);

        assert_eq!(
            count_solutions(&board),
            1
        );
    }

    #[test]
    fn test_count_solutions_empty_board() {
        let board = [[0u8; 9]; 9];
        print_board(&board);

        assert!(
            count_solutions(&board) > 1
        );
    }

    #[test]
    fn test_count_givens_full(){
        let board = generate_full_board();
        assert_eq!(count_givens(&board), 81);
    }

    #[test]
    fn test_count_givens_empty(){
        let board = [[0u8; 9]; 9];
        assert_eq!(count_givens(&board), 0);
    }

    #[test]
    fn test_dig_holes_target(){
        let mut board =
            generate_full_board();

        dig_holes(
            &mut board,
            10,
        );
        print_board(&board);
        println!("Fields left: {}", count_givens(&board));
        assert!(has_unique_solution(&board))
    }
}
