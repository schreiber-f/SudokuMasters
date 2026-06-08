pub mod singles;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Technique {
    NakedSingle,
    HiddenSingle,
}