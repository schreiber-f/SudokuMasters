use crate::strategies::triples::{find_hidden_triple_in_unit, find_naked_triple_in_unit};
use crate::strategies::cell_utils::{row_cells, col_cells, box_cells};


#[test]
fn test_hidden_triple_row() {

    let mut candidates = [[0u16;9];9];

    candidates[0][0] =
        (1<<0)|(1<<1)|(1<<2)|(1<<6);

    candidates[0][3] =
        (1<<0)|(1<<1)|(1<<2)|(1<<5);

    candidates[0][6] =
        (1<<0)|(1<<1)|(1<<2)|(1<<8);

    assert!(
        find_hidden_triple_in_unit(
            &mut candidates,
            &row_cells(0),
        )
    );

    let expected =
        (1<<0)|(1<<1)|(1<<2);

    assert_eq!(candidates[0][0], expected);
    assert_eq!(candidates[0][3], expected);
    assert_eq!(candidates[0][6], expected);
}

#[test]
fn test_hidden_triple_column() {

    let mut candidates = [[0u16;9];9];

    candidates[0][2] =
        (1<<3)|(1<<4)|(1<<5)|(1<<0);

    candidates[3][2] =
        (1<<3)|(1<<4)|(1<<5)|(1<<8);

    candidates[8][2] =
        (1<<3)|(1<<4)|(1<<5)|(1<<7);

    assert!(
        find_hidden_triple_in_unit(
            &mut candidates,
            &col_cells(2),
        )
    );

    let expected =
        (1<<3)|(1<<4)|(1<<5);

    assert_eq!(candidates[0][2], expected);
    assert_eq!(candidates[3][2], expected);
    assert_eq!(candidates[8][2], expected);
}

#[test]
fn test_hidden_triple_box() {

    let mut candidates = [[0u16;9];9];

    candidates[0][0] =
        (1<<2)|(1<<4)|(1<<8)|(1<<1);

    candidates[1][1] =
        (1<<2)|(1<<4)|(1<<8)|(1<<6);

    candidates[2][2] =
        (1<<2)|(1<<4)|(1<<8)|(1<<7);

    assert!(
        find_hidden_triple_in_unit(
            &mut candidates,
            &box_cells(0),
        )
    );

    let expected =
        (1<<2)|(1<<4)|(1<<8);

    assert_eq!(candidates[0][0], expected);
    assert_eq!(candidates[1][1], expected);
    assert_eq!(candidates[2][2], expected);
}

#[test]
fn test_hidden_triple_none() {

    let mut candidates = [[0u16;9];9];

    candidates[0][0] = (1<<0)|(1<<1);
    candidates[0][1] = (1<<1)|(1<<2);
    candidates[0][2] = (1<<2)|(1<<3);

    assert!(
        !find_hidden_triple_in_unit(
            &mut candidates,
            &row_cells(0),
        )
    );
}


#[test]
fn test_naked_triple_row() {
    let mut candidates = [[0u16; 9]; 9];

    let a = (1 << 0) | (1 << 1);          // {1,2}
    let b = (1 << 1) | (1 << 2);          // {2,3}
    let c = (1 << 0) | (1 << 2);          // {1,3}

    candidates[0][0] = a;
    candidates[0][1] = b;
    candidates[0][2] = c;

    candidates[0][5] =
        (1 << 0) |
            (1 << 1) |
            (1 << 2) |
            (1 << 5);

    assert!(find_naked_triple_in_unit(
        &mut candidates,
        &row_cells(0),
    ));

    assert_eq!(
        candidates[0][5],
        1 << 5
    );
}


#[test]
fn test_naked_triple_column() {
    let mut candidates = [[0u16; 9]; 9];

    candidates[0][0] = (1 << 3) | (1 << 4);
    candidates[3][0] = (1 << 4) | (1 << 5);
    candidates[5][0] = (1 << 3) | (1 << 5);

    candidates[8][0] =
        (1 << 3) |
            (1 << 4) |
            (1 << 5) |
            (1 << 8);

    assert!(find_naked_triple_in_unit(
        &mut candidates,
        &col_cells(0),
    ));

    assert_eq!(
        candidates[8][0],
        1 << 8
    );
}


#[test]
fn test_naked_triple_box() {
    let mut candidates = [[0u16; 9]; 9];

    candidates[0][0] = (1 << 5) | (1 << 6);
    candidates[1][1] = (1 << 6) | (1 << 7);
    candidates[2][2] = (1 << 5) | (1 << 7);

    candidates[0][1] =
        (1 << 5) |
            (1 << 6) |
            (1 << 7) |
            (1 << 1);

    assert!(find_naked_triple_in_unit(
        &mut candidates,
        &box_cells(0),
    ));

    assert_eq!(
        candidates[0][1],
        1 << 1
    );
}

#[test]
fn test_naked_triple_none() {
    let mut candidates = [[0u16; 9]; 9];

    candidates[0][0] = (1 << 0) | (1 << 1);
    candidates[0][1] = (1 << 1) | (1 << 2);
    candidates[0][2] = (1 << 2) | (1 << 3);

    assert!(
        !find_naked_triple_in_unit(
            &mut candidates,
            &row_cells(0),
        )
    );
}