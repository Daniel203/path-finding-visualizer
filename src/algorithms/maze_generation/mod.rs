use std::fmt::Display;

use strum_macros::EnumIter;

pub mod binary_tree;
pub mod dfs;
pub mod recursive_division;

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq)]
pub enum MGAlgorithms {
    NotSelected,
    BinaryTree,
    Dfs,
    RecursiveDivision,
}

impl Display for MGAlgorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MGAlgorithms::NotSelected => write!(f, "Maze-Generation Algorithm"),
            MGAlgorithms::BinaryTree => write!(f, "Binary Tree"),
            MGAlgorithms::Dfs => write!(f, "DFS"),
            MGAlgorithms::RecursiveDivision => write!(f, "Recursive Division"),
        }
    }
}
