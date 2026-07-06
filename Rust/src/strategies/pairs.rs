use crate::candidates::{Candidates, count_bits, remove_candidates};
use std::collections::HashMap;
use crate::strategies::cell_utils::{row_cells, col_cells, box_cells};

pub fn find_naked_pairs_in_unit(candidates: &mut Candidates, unit_cells: &[(usize, usize)]) -> bool {
    let mut changed = false;
    let mut mask_counts: HashMap<u16,Vec<(usize, usize)>> = HashMap::new();

    // 1. Group cells by their candidate masks and save their indices within the unit
    for &(r, c) in unit_cells {
        let mask = candidates[r][c];


        if count_bits(mask) == 2 {
            mask_counts.entry(mask).or_default().push((r, c));
        }
    }

    // 2. Look for masks that appear exactly 2 times in the unit
    for (&mask, indices) in &mask_counts {
        if indices.len() == 2 {
            let cell1 = indices[0];
            let cell2 = indices[1];

            // Eliminate the candidates of the pair
            for &(r, c) in unit_cells {
                // Skip the two cells that form the naked pair
                if (r, c) == cell1 || (r, c) == cell2 {
                    continue;
                }

                // Check if this cell contains any of our pair's candidates and remove them
                if (candidates[r][c] & mask) > 0 {
                    changed |= remove_candidates(candidates, r, c, mask);
                }
            }
        }
    }

    changed
}


pub fn find_naked_pairs_in_unit_fast(candidates: &mut Candidates, unit_cells: &[(usize, usize)]) -> bool {
    let mut changed = false;
    let len = unit_cells.len();

    // 1. Suche das erste Element des Paares über den Array-Index
    for i in 0..(len - 1) {
        let (r1, c1) = unit_cells[i];
        let mask1 = candidates[r1][c1];

        if count_bits(mask1) != 2 {
            continue;
        }

        // 2. Suche das zweite Element des Paares weiter hinten im Array
        for j in (i + 1)..len {
            let (r2, c2) = unit_cells[j];
            let mask2 = candidates[r2][c2];

            // Ein Naked Pair wurde gefunden
            if mask1 == mask2 {
                // 3. Alle anderen Zellen im unit_cells-Array bereinigen
                for k in 0..len {
                    if k == i || k == j {
                        continue; // Überspringe die beiden Pair-Zellen
                    }

                    let (r_other, c_other) = unit_cells[k];

                    if (candidates[r_other][c_other] & mask1) > 0 {
                        changed |= remove_candidates(candidates, r_other, c_other, mask1);
                    }
                }
            }
            if changed {
                return true;
            }
        }
    }

    changed
}


pub fn find_hidden_pairs_in_unit(
    candidates: &mut Candidates,
    unit_cells: &[(usize, usize)]
) -> bool {
    let mut changed = false;

    // Jedes Element speichert als Bitmaske, in welchen der 9 Zellen die Zahl vorkommt
    let mut digit_positions = [0u16; 9];

    // 1. Schritt: Positionen für jede Zahl (1-9) in der Unit sammeln
    for cell_idx in 0..9 {
        let (r, c) = unit_cells[cell_idx];
        let mask = candidates[r][c];

        for digit_idx in 0..9 {
            // Prüfen, ob das Bit für diese Zahl in der Zelle gesetzt ist
            if (mask & (1 << digit_idx)) != 0 {
                // Setze das Bit für den aktuellen cell_idx
                digit_positions[digit_idx] |= 1 << cell_idx;
            }
        }
    }

    // 2. Schritt: Vergleiche die Positions-Masken der Zahlen miteinander
    for d1 in 0..8 {
        let pos_mask1 = digit_positions[d1];

        // Ein Hidden Pair benötigt exakt 2 Vorkommen in der Unit
        if count_bits(pos_mask1) != 2 {
            continue;
        }

        for d2 in (d1 + 1)..9 {
            let pos_mask2 = digit_positions[d2];

            // Wenn beide Zahlen in exakt denselben zwei Zellen vorkommen
            if pos_mask1 == pos_mask2{

                // Die Maske der erlaubten Zahlen für das Hidden Pair.
                let pair_mask = (1 << d1) | (1 << d2);

                for cell_idx in 0..9 {
                    if (pos_mask1 & (1 << cell_idx)) != 0 {
                        let (r, c) = unit_cells[cell_idx];

                        // Berechnen, was gelöscht werden muss
                        let to_remove = candidates[r][c] & !pair_mask;

                        if to_remove > 0 {
                            changed |= remove_candidates(candidates, r, c, to_remove);
                        }
                    }
                }
            }
            if changed {
                return true;
            }
        }
    }

    changed
}

pub fn apply_naked_pair(
    candidates: &mut Candidates,
)->bool{
    for row in 0..9 {
        let cells = row_cells(row);
        
        if find_naked_pairs_in_unit_fast(candidates, &cells) {
            return true;
        }
    }

    for col in 0..9 {
        let cells = col_cells(col);

        if find_naked_pairs_in_unit_fast(candidates, &cells) {
            return true;
        }
    }
    
    for box_idx in 0..9 {
        let cells = box_cells(box_idx);
        
        if find_naked_pairs_in_unit_fast(candidates, &cells) {
            return true;
        }
    }
    
    false
}

pub fn apply_hidden_pair(
    candidates: &mut Candidates,
) -> bool {

    for row in 0..9 {
        let cells = row_cells(row);

        if find_hidden_pairs_in_unit(
            candidates,
            &cells
        ) {
            return true;
        }
    }

    for col in 0..9 {
        let cells = col_cells(col);

        if find_hidden_pairs_in_unit(
            candidates,
            &cells
        ) {
            return true;
        }
    }

    for box_idx in 0..9 {
        let cells = box_cells(box_idx);

        if find_hidden_pairs_in_unit(
            candidates,
            &cells
        ) {
            return true;
        }
    }

    false
}