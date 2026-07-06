use crate::strategies::rectangles::{apply_unique_rectangle_type1};

#[test]
fn test_unique_rectangle_type1(){

    let mut candidates =
        [[0u16;9];9];


    let pair =
        (1<<1) |
            (1<<8); // {2,9}


    let five =
        1<<4;


    // drei reine Paare

    candidates[3][3]=pair;

    candidates[3][6]=pair;

    candidates[6][3]=pair;



    // vierte Ecke

    candidates[6][6]=
        pair | five;



    assert!(
        apply_unique_rectangle_type1(
            &mut candidates
        )
    );


    assert_eq!(
        candidates[6][6],
        five
    );

}


#[test]
fn test_no_unique_rectangle(){

    let mut candidates =
        [[0u16;9];9];


    let pair =
        (1<<1)|(1<<8);


    candidates[3][3]=pair;
    candidates[3][6]=pair;
    candidates[6][3]=pair;


    candidates[6][6]=
        pair |
            (1<<4) |
            (1<<5);


    assert!(
        !apply_unique_rectangle_type1(
            &mut candidates
        )
    );

}