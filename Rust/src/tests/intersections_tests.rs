use crate::strategies::cell_utils::{row_cells};
use crate::strategies::intersections::{find_pointing_pair_in_box, apply_pointing_pair, find_claiming_in_unit, apply_box_line_reduction};

#[test]
fn test_pointing_pair_row() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;

    candidates[0][0] = five;
    candidates[0][2] = five;

    candidates[0][4] =
        five |
            (1 << 7);

    assert!(
        find_pointing_pair_in_box(
            &mut candidates,
            0
        )
    );

    assert_eq!(
        candidates[0][4],
        1 << 7
    );
}

#[test]
fn test_pointing_pair_column() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;

    candidates[0][0] = five;
    candidates[2][0] = five;

    candidates[5][0] =
        five |
            (1 << 7);

    assert!(
        find_pointing_pair_in_box(
            &mut candidates,
            0
        )
    );

    assert_eq!(
        candidates[5][0],
        1 << 7
    );
}

#[test]
fn test_pointing_pair_none() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;

    candidates[0][0] = five;
    candidates[1][2] = five;

    assert!(
        !find_pointing_pair_in_box(
            &mut candidates,
            0
        )
    );
}

#[test]
fn test_apply_pointing_pair() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;
    let eight = 1 << 7;

    candidates[0][0] = five;
    candidates[0][2] = five;

    candidates[0][4] = five | eight;

    assert!(
        apply_pointing_pair(&mut candidates)
    );

    assert_eq!(
        candidates[0][4],
        eight
    );
}


#[test]
fn claiming_pair_row() {
    let mut candidates = [[0u16;9];9];

    let five = 1 << 4;
    let eight = 1 << 7;

    candidates[0][0] = five;
    candidates[0][2] = five;

    candidates[1][1] = five | eight;
    candidates[2][2] = five | eight;

    assert!(
        find_claiming_in_unit(
            &mut candidates,
            &row_cells(0),
        )
    );

    assert_eq!(
        candidates[1][1],
        eight
    );

    assert_eq!(
        candidates[2][2],
        eight
    );
}

#[test]
fn claiming_triple_row() {
    let mut candidates = [[0u16;9];9];

    let five = 1 << 4;
    let eight = 1 << 7;

    candidates[0][0] = five;
    candidates[0][1] = five;
    candidates[0][2] = five;

    candidates[1][1] = five | eight;
    candidates[2][2] = five | eight;

    assert!(
        find_claiming_in_unit(
            &mut candidates,
            &row_cells(0),
        )
    );

    assert_eq!(
        candidates[1][1],
        eight
    );

    assert_eq!(
        candidates[2][2],
        eight
    );
}

#[test]
fn apply_box_line_reduction_row() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;
    let eight = 1 << 7;

    // Row 0: 5 nur innerhalb Box 0
    candidates[0][0] = five;
    candidates[0][2] = five;

    // In derselben Box existieren weitere 5en
    candidates[1][1] = five | eight;
    candidates[2][2] = five | eight;

    assert!(apply_box_line_reduction(&mut candidates));

    assert_eq!(candidates[1][1], eight);
    assert_eq!(candidates[2][2], eight);
}

#[test]
fn apply_box_line_reduction_no_claiming() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;

    candidates[0][0] = five;
    candidates[0][4] = five;

    assert!(!apply_box_line_reduction(&mut candidates));
}

#[test]
fn apply_box_line_reduction_triple() {
    let mut candidates = [[0u16; 9]; 9];

    let four = 1 << 3;
    let nine = 1 << 8;

    candidates[0][0] = four;
    candidates[0][1] = four;
    candidates[0][2] = four;

    candidates[1][1] = four | nine;
    candidates[2][2] = four | nine;

    assert!(apply_box_line_reduction(&mut candidates));

    assert_eq!(candidates[1][1], nine);
    assert_eq!(candidates[2][2], nine);
}
