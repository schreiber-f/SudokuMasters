use crate::board::{Board, board_is_complete};
use crate::candidates::{compute_candidates};
use crate::strategies::singles::{apply_naked_single, apply_hidden_single};
use crate::strategies::pairs::{apply_naked_pair, apply_hidden_pair};
use crate::strategies::triples::{apply_naked_triple, apply_hidden_triple};
use crate::strategies::intersections::{apply_pointing_pair, apply_box_line_reduction};
use crate::strategies::bug::{apply_bug_plus_one};
use crate::strategies::fish::{apply_x_wing};
use crate::strategies::rectangles::{apply_unique_rectangle_type1};
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

        if apply_box_line_reduction(&mut candidates) {
            *counts
                .entry(Technique::BoxLineReduction)
                .or_insert(0) += 1;

            continue;
        }

        if apply_naked_triple(&mut candidates) {
            *counts
                .entry(Technique::NakedTriple)
                .or_insert(0) += 1;

            continue;
        }
        println!("no naked triple found");

        if apply_hidden_triple(&mut candidates) {
            *counts
                .entry(Technique::HiddenTriple)
                .or_insert(0) += 1;

            continue;
        }
        println!("no hidden triple found");

        if apply_bug_plus_one(board,&mut candidates) {
            *counts
                .entry(Technique::BugPlus1)
                .or_insert(0) += 1;

            continue;
        }
        println!("no bug+1 found");

        if apply_x_wing(&mut candidates) {
            *counts
                .entry(Technique::XWing)
                .or_insert(0) += 1;

            continue;
        }
        println!("no X-Wing found");

        if apply_unique_rectangle_type1(&mut candidates) {
            *counts
                .entry(Technique::UniqueRectangleT1)
                .or_insert(0) += 1;

            continue;
        }
        println!("no unique rectangle type 1 found");

        break;
    }

    SolveReport {
        solved_board: *board,
        is_solved: board_is_complete(board),
        technique_counts: counts,
    }
}
