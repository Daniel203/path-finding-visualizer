use std::fmt::Display;

use strum_macros::EnumIter;
use yewdux::prelude::Dispatch;

use crate::components::store::matrix_state::MatrixState;

use self::{a_star::a_star, a_star_search::a_star_search, bfs::bfs, dfs::dfs};

pub mod a_star;
pub mod a_star_search;
pub mod bfs;
pub mod dfs;

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq)]
pub enum PFAlgorithms {
    NotSelected,
    Bfs,
    Dfs,
    AStar,
    AStarSearch,
}

impl Default for PFAlgorithms {
    fn default() -> Self {
        return Self::NotSelected;
    }
}

impl Display for PFAlgorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PFAlgorithms::NotSelected => write!(f, "Path-Finding Algorithm"),
            PFAlgorithms::Bfs => write!(f, "BFS"),
            PFAlgorithms::Dfs => write!(f, "DFS"),
            PFAlgorithms::AStar => write!(f, "A*"),
            PFAlgorithms::AStarSearch => write!(f, "A* Search"),
        }
    }
}

type PFFunction = fn(&Dispatch<MatrixState>) -> Option<i32>;

impl PFAlgorithms {
    pub fn get_function(self) -> Option<PFFunction> {
        return match self {
            PFAlgorithms::NotSelected => None,
            PFAlgorithms::Bfs => Some(bfs),
            PFAlgorithms::Dfs => Some(dfs),
            PFAlgorithms::AStar => Some(a_star),
            PFAlgorithms::AStarSearch => Some(a_star_search),
        };
    }
}
