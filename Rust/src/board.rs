pub type Board = [[u8; 9]; 9];

pub fn is_valid(
    board: &Board,
    row: usize,
    col: usize,
    num: u8
) -> bool{

    // check row
    for i in 0..9 {
        if board[row][i] == num {
            return false;
        }
    }

    // check col
    for i in 0..9 {
        if board[i][col] == num {
            return false;
        }
    }

    // check box
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;

    for i in start_row..start_row + 3 {
        for j in start_col..start_col + 3 {
            if board[i][j] == num {
                return false;
            }
        }
    }

    true
}

pub fn full_board_valid(board: &Board) -> bool {

    // rows and cols

    for i in 0..9 {
        let mut seen_row = [false; 9];
        let mut seen_col = [false; 9];

        for j in 0..9 {
            // check row
            let row_cell = board[i][j];
            if row_cell != 0 {
                let index = (row_cell as usize) - 1;
                if seen_row[index] { return false; }
                seen_row[index] = true;
            }else{
                println!("board is not full");
                return false
            }

            // check col
            let col_cell = board[j][i];
            if col_cell != 0 {
                let index = (col_cell as usize) - 1;
                if seen_col[index] { return false; }
                seen_col[index] = true;
            }else{
                println!("board is not full");
                return false
            }
        }
    }

    //blocks
    for block_r in (0..9).step_by(3) {
        for block_c in (0..9).step_by(3) {
            let mut seen_block = [false; 9];

            for i in 0..3 {
                for j in 0..3 {
                    let cell = board[block_r + i][block_c + j];
                    if cell != 0 {
                        let index = (cell as usize) - 1;
                        if seen_block[index] { return false; }
                        seen_block[index] = true;
                    }else{
                        println!("board is not full");
                        return false
                    }
                }
            }
        }
    }

    true
}

pub fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
    println!();
}