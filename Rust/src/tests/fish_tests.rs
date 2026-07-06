use crate::strategies::fish::{find_x_wing_rows, find_x_wing_cols, apply_x_wing};

#[test]
fn test_x_wing_rows() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;
    let eight = 1 << 7;

    // X-Wing
    candidates[1][2] = five;
    candidates[1][6] = five;

    candidates[5][2] = five;
    candidates[5][6] = five;

    // Zu eliminieren
    candidates[3][2] = five | eight;
    candidates[7][6] = five | eight;

    assert!(find_x_wing_rows(&mut candidates));

    assert_eq!(candidates[3][2], eight);
    assert_eq!(candidates[7][6], eight);
}

#[test]
fn test_x_wing_cols() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;
    let eight = 1 << 7;

    candidates[2][1] = five;
    candidates[6][1] = five;

    candidates[2][5] = five;
    candidates[6][5] = five;

    candidates[2][3] = five | eight;
    candidates[6][7] = five | eight;

    assert!(find_x_wing_cols(&mut candidates));

    assert_eq!(candidates[2][3], eight);
    assert_eq!(candidates[6][7], eight);
}

#[test]
fn test_x_wing_none() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;

    candidates[1][2] = five;
    candidates[1][6] = five;

    candidates[5][2] = five;
    candidates[5][7] = five;

    assert!(!find_x_wing_rows(&mut candidates));
}

#[test]
fn test_apply_x_wing() {
    let mut candidates = [[0u16; 9]; 9];

    let five = 1 << 4;
    let eight = 1 << 7;

    candidates[1][2] = five;
    candidates[1][6] = five;

    candidates[5][2] = five;
    candidates[5][6] = five;

    candidates[3][2] = five | eight;

    assert!(apply_x_wing(&mut candidates));

    assert_eq!(candidates[3][2], eight);
}