use crate::board::{Board, board_is_complete};
use crate::candidates::{compute_candidates};
use crate::strategies::singles::{apply_naked_single, apply_hidden_single};
use crate::strategies::pairs::{apply_naked_pair, apply_hidden_pair};
use crate::strategies::intersections::{apply_pointing_pair};
use crate::strategies::{Technique};
use std::collections::HashMap;


#[derive(Debug)]
pub struct SolveReport{
    pub solved_board: Board,
    pub is_solved: bool,
    pub technique_counts: HashMap<Technique, usize>,
}


pub fn human_solve(board: &mut Board) -> SolveReport{
    let mut candidates =
        compute_candidates(board);

    let mut counts = HashMap::new();

    loop {
        if apply_naked_single(board, &mut candidates) {
            *counts
                .entry(Technique::NakedSingle)
                .or_insert(0) += 1;

            continue;
        }
        println!("no naked single found");
        if apply_hidden_single(board, &mut candidates) {

            *counts
                .entry(Technique::HiddenSingle)
                .or_insert(0) += 1;

            continue;
        }
        println!("no hidden single found");
        
        if apply_naked_pair(&mut candidates) {
            *counts
                .entry(Technique::NakedPair)
                .or_insert(0) += 1;
            
            continue;
        }
        println!("no naked pair found");
        
        if apply_hidden_pair(&mut candidates) {
            *counts
                .entry(Technique::HiddenPair)
                .or_insert(0) += 1;
            
            continue;
        }
        println!("no hidden pair found");
        
        if apply_pointing_pair(&mut candidates){
            *counts
                .entry(Technique::PointingPair)
                .or_insert(0) += 1;
            
            continue;
        }
        println!("no pointing pair found");

        break;
    }

    SolveReport {
        solved_board: *board,
        is_solved: board_is_complete(board),
        technique_counts: counts,
    }
}
