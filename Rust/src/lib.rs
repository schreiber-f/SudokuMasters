pub mod board;
pub mod solver;
pub mod generator;
pub mod human_solver;
pub mod strategies;
pub mod candidates;

#[cfg(test)]
mod tests {
    use crate::board::{is_valid, Board, print_board, full_board_valid, board_is_complete};
    use crate::solver::{find_empty, solve, count_solutions, has_unique_solution, solve_random};
    use crate::generator::{count_givens, generate_full_board, dig_holes};
    use crate::candidates::{count_bits, single_candidate, compute_candidates, apply_value};
    use crate::human_solver::human_solve;
    use crate::strategies::singles::{apply_hidden_single, apply_naked_single};
    use crate::strategies::pairs::{find_naked_pairs_in_unit_fast, find_hidden_pairs_in_unit};
    use crate::strategies::cell_utils::{row_cells, col_cells, box_cells};
    //use super::*;

    #[test]
    fn test_invalid_row() {
        println!("testing invalid_row");
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
        println!("testing invalid_column");
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
        println!("testing invalid_block");
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
        println!("testing valid_move");

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
        println!("testing find_empty");

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

    #[test]
    fn test_random_solver() {
        println!("testing random_solver");
        let board = generate_full_board();

        print_board(&board);

        assert!(full_board_valid(&board));
    }

    //#[test]
    //fn test_generate_multiple_boards() {
    //    for _ in 0..100 {
    //        let board = generate_full_board();

    //        assert!(full_board_valid(&board));
    //    }
    //}

    #[test]
    fn test_count_solutions_full_board() {
        println!("testing count_solutions_full_board");
        let board = generate_full_board();
        print_board(&board);

        assert_eq!(
            count_solutions(&board),
            1
        );
    }

    #[test]
    fn test_count_solutions_empty_board() {
        println!("testing count_solutions_empty_board");
        let board = [[0u8; 9]; 9];
        print_board(&board);

        assert!(
            count_solutions(&board) > 1
        );
    }

    #[test]
    fn test_count_givens_full(){
        println!("testing count_givens_full");
        let board = generate_full_board();
        assert_eq!(count_givens(&board), 81);
    }

    #[test]
    fn test_count_givens_empty(){
        println!("testing count_givens_empty");
        let board = [[0u8; 9]; 9];
        assert_eq!(count_givens(&board), 0);
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

    #[test]
    fn test_board_is_complete_true() {
        let board = [[1u8; 9]; 9];

        assert!(board_is_complete(&board));
    }

    #[test]
    fn test_board_is_complete_false() {
        let mut board = [[1u8; 9]; 9];

        board[3][4] = 0;

        assert!(!board_is_complete(&board));
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
    fn test_naked_pair_row() {
        let mut candidates = [[0u16; 9]; 9];

        let pair = (1 << 1) | (1 << 6); // {2,7}

        candidates[0][0] = pair;
        candidates[0][1] = pair;

        candidates[0][2] =
            pair |
                (1 << 0) |
                (1 << 4);

        let unit = [
            (0,0),(0,1),(0,2),
            (0,3),(0,4),(0,5),
            (0,6),(0,7),(0,8),
        ];

        assert!(
            find_naked_pairs_in_unit_fast(
                &mut candidates,
                &unit
            )
        );

        assert_eq!(
            candidates[0][2],
            (1 << 0) | (1 << 4)
        );
    }

    #[test]
    fn test_naked_pair_column() {
        let mut candidates = [[0u16; 9]; 9];

        let pair = (1 << 2) | (1 << 5); // {3,6}

        candidates[0][0] = pair;
        candidates[1][0] = pair;

        candidates[2][0] =
            pair |
                (1 << 7);

        let unit = [
            (0,0),(1,0),(2,0),
            (3,0),(4,0),(5,0),
            (6,0),(7,0),(8,0),
        ];

        assert!(
            find_naked_pairs_in_unit_fast(
                &mut candidates,
                &unit
            )
        );

        assert_eq!(
            candidates[2][0],
            (1 << 7)
        );
    }

    #[test]
    fn test_naked_pair_box() {
        let mut candidates = [[0u16; 9]; 9];

        let pair =
            (1 << 0) |
                (1 << 8); // {1,9}

        candidates[0][0] = pair;
        candidates[1][1] = pair;

        candidates[2][2] =
            pair |
                (1 << 4);

        let unit = [
            (0,0),(0,1),(0,2),
            (1,0),(1,1),(1,2),
            (2,0),(2,1),(2,2),
        ];

        assert!(
            find_naked_pairs_in_unit_fast(
                &mut candidates,
                &unit
            )
        );

        assert_eq!(
            candidates[2][2],
            (1 << 4)
        );
    }

    #[test]
    fn test_naked_pair_none() {
        let mut candidates = [[0u16; 9]; 9];

        candidates[0][0] =
            (1 << 0) |
                (1 << 1);

        candidates[0][1] =
            (1 << 0) |
                (1 << 2);

        let unit = [
            (0,0),(0,1),(0,2),
            (0,3),(0,4),(0,5),
            (0,6),(0,7),(0,8),
        ];

        assert!(
            !find_naked_pairs_in_unit_fast(
                &mut candidates,
                &unit
            )
        );
    }

    #[test]
    fn test_hidden_pair_row() {
        let mut candidates = [[0u16; 9]; 9];

        candidates[0][0] =
            (1 << 0) |
                (1 << 1) |
                (1 << 6);

        candidates[0][3] =
            (1 << 1) |
                (1 << 3) |
                (1 << 6);

        let unit = [
            (0,0),(0,1),(0,2),
            (0,3),(0,4),(0,5),
            (0,6),(0,7),(0,8),
        ];

        assert!(
            find_hidden_pairs_in_unit(
                &mut candidates,
                &unit
            )
        );

        let expected =
            (1 << 1) |
                (1 << 6);

        assert_eq!(
            candidates[0][0],
            expected
        );

        assert_eq!(
            candidates[0][3],
            expected
        );
    }

    #[test]
    fn test_hidden_pair_column() {
        let mut candidates = [[0u16; 9]; 9];

        candidates[0][0] =
            (1 << 2) |
                (1 << 4) |
                (1 << 8);

        candidates[5][0] =
            (1 << 4) |
                (1 << 7) |
                (1 << 8);

        let unit = [
            (0,0),(1,0),(2,0),
            (3,0),(4,0),(5,0),
            (6,0),(7,0),(8,0),
        ];

        assert!(
            find_hidden_pairs_in_unit(
                &mut candidates,
                &unit
            )
        );

        let expected =
            (1 << 4) |
                (1 << 8);

        assert_eq!(
            candidates[0][0],
            expected
        );

        assert_eq!(
            candidates[5][0],
            expected
        );
    }

    #[test]
    fn test_hidden_pair_none() {
        let mut candidates = [[0u16; 9]; 9];

        candidates[0][0] =
            (1 << 0) |
                (1 << 1);

        candidates[0][1] =
            (1 << 0) |
                (1 << 2);

        let unit = [
            (0,0),(0,1),(0,2),
            (0,3),(0,4),(0,5),
            (0,6),(0,7),(0,8),
        ];

        assert!(
            !find_hidden_pairs_in_unit(
                &mut candidates,
                &unit
            )
        );
    }

    #[test]
    fn test_row_cells() {
        let cells = row_cells(3);

        assert_eq!(cells[0], (3, 0));
        assert_eq!(cells[8], (3, 8));
    }

    #[test]
    fn test_col_cells() {
        let cells = col_cells(4);

        assert_eq!(cells[0], (0, 4));
        assert_eq!(cells[8], (8, 4));
    }

    #[test]
    fn test_box_cells() {
        let cells = box_cells(4);

        assert_eq!(cells[0], (3, 3));
        assert_eq!(cells[8], (5, 5));
    }

    #[test]
    fn test_human_solver_singles_pairs() {
        println!("testing human_solver_singles_and_pairs...");
        let mut board = generate_full_board();
        println!("board before:");
        print_board(&board);
        dig_holes(&mut board, 30);

        println!("board with holes:");
        print_board(&board);

        let report = human_solve(&mut board);

        assert!(report.is_solved);
        println!("board after:");
        print_board(&board);
        println!("solve report {:?}", report);
    }
}
