use crate::algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Copy)]
pub struct AlgorithmSelectorState {
    pub selected_pf_algorithm: PFAlgorithms,
    pub selected_mg_algorithm: MGAlgorithms,
    pub find_path_clicked: bool,
    pub generate_maze_clicked: bool,
    pub reset_board_clicked: bool,
    pub reset_visited_clicked: bool,
}
