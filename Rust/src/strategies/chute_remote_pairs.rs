

pub fn apply_chute_remote_pair(candidates: &mut Candidates) -> bool {
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            let mask = candidates[r][c];
            if count_bits(mask) == 2 {
                pairs.push((r, c));
            }
        }
    }

    for i in 0 .. pairs.len()-1{
        for j in i+1 .. pairs.len() {
            let (r1, c1) = pairs[i];
            let (r2, c2) = pairs[j];

            if r1 == r2 || c1 == c2 {
                continue;
            }

            let mask1 = candidates[r1][c1];
            let mask2 = candidates[r2][c2];

            if mask1 != mask2 {
                continue;
            }

            let common_candidates = mask1;

            let mut extra_cells: Vec<(usize, usize)> = Vec::new();

            for r in 0..9 {
                for c in 0..9 {
                    if (r == r1 && c == c1) || (r == r2 && c == c2) {
                        continue;
                    }
                    if candidates[r][c] & common_candidates != 0 {
                        extra_cells.push((r, c));
                    }
                }
            }

            if extra_cells.len() == 0 {
                continue;
            }

            for &(er, ec) in &extra_cells {
                candidates[er][ec] &= !common_candidates;
            }

            return true;
        }
    }

    false
}