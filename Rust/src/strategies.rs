pub mod singles;
pub mod pairs;
pub mod cell_utils;
pub mod intersections;
pub mod triples;
pub mod fish;
pub mod wing;
pub mod bug;
pub mod rectangles;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Technique {
    NakedSingle,
    HiddenSingle,
    NakedPair,
    HiddenPair,
    NakedTriple,
    HiddenTriple,
    PointingPair,
    BoxLineReduction,
    XWing,
    BugPlus1
}