use std::fmt::Display;

use strum_macros::EnumIter;

pub mod a_star;
pub mod a_star_search;
pub mod bfs;
pub mod dfs;

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq)]
pub enum PFAlgorithms {
    NotSelected,
    BFS,
    DFS,
    AStar,
    AStarSearch,
}

impl Display for PFAlgorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PFAlgorithms::NotSelected => write!(f, "Select an algorithm"),
            PFAlgorithms::BFS => write!(f, "BFS"),
            PFAlgorithms::DFS => write!(f, "DFS"),
            PFAlgorithms::AStar => write!(f, "A*"),
            PFAlgorithms::AStarSearch => write!(f, "A* Search"),
        }
    }
}
