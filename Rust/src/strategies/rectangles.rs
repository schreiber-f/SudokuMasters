use crate::candidates::{
    Candidates,
    count_bits,
    remove_candidates
};


pub fn apply_unique_rectangle_type1(
    candidates:&mut Candidates
)->bool{
    for r1 in 0..8 {
        for r2 in r1+1..9 {

            for c1 in 0..8 {
                for c2 in c1+1..9 {


                    let coords=[
                        (r1,c1),
                        (r1,c2),
                        (r2,c1),
                        (r2,c2)
                    ];


                    let masks=[
                        candidates[r1][c1],
                        candidates[r1][c2],
                        candidates[r2][c1],
                        candidates[r2][c2],
                    ];


                    // Kandidatenpaar suchen

                    for pair_index in 0..4 {

                        let pair=masks[pair_index];


                        if count_bits(pair)!=2 {
                            continue;
                        }


                        let mut pair_cells=0;
                        let mut extra_cell=None;


                        for i in 0..4 {

                            if masks[i]==pair {
                                pair_cells+=1;
                            }

                            else if
                            masks[i]&pair==pair
                                &&
                                count_bits(masks[i])>2
                            {
                                extra_cell=Some(coords[i]);
                            }
                        }


                        if pair_cells==3 {

                            if let Some((r,c))=extra_cell {

                                if remove_candidates(
                                    candidates,
                                    r,
                                    c,
                                    pair
                                ){
                                    return true;
                                }
                            }

                        }

                    }

                }
            }
        }
    }


    false
}