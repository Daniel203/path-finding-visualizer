use std::fmt::Display;

use strum_macros::EnumIter;
use yewdux::prelude::Dispatch;

use crate::components::store::matrix_state::MatrixState;

use self::{binary_tree::binary_tree, dfs::dfs, recursive_division::recursive_division};

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

impl Default for MGAlgorithms {
    fn default() -> Self {
        return Self::NotSelected;
    }
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

type MGFunction = fn(&Dispatch<MatrixState>);

impl MGAlgorithms {
    pub fn get_function(self) -> Option<MGFunction> {
        return match self {
            MGAlgorithms::NotSelected => None,
            MGAlgorithms::BinaryTree => Some(binary_tree),
            MGAlgorithms::Dfs => Some(dfs),
            MGAlgorithms::RecursiveDivision => Some(recursive_division),
        };
    }
}
