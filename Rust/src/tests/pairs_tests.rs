use crate::strategies::pairs::{find_naked_pairs_in_unit_fast, find_hidden_pairs_in_unit};

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