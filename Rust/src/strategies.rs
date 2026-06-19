pub mod singles;
pub mod pairs;
pub mod cell_utils;
pub mod intersections;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Technique {
    NakedSingle,
    HiddenSingle,
    NakedPair,
    HiddenPair,
}