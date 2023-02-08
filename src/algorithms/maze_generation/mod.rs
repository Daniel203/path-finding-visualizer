use std::fmt::Display;

use strum_macros::EnumIter;

pub mod binary_tree;

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq)]
pub enum MGAlgorithms {
    NotSelected,
    BinaryTree,
}

impl Display for MGAlgorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MGAlgorithms::NotSelected => write!(f, "Select an algorithm"),
            MGAlgorithms::BinaryTree => write!(f, "Binary Tree"),
        }
    }
}
