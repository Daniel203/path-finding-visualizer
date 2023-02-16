use yew::{function_component, html, use_state, Callback, Html};

use crate::algorithms::maze_generation::MGAlgorithms;
use crate::algorithms::path_finding::PFAlgorithms;
use crate::components::algorithm_selector_component::AlgorithmSelectorComponent;
use crate::components::matrix_component::MatrixComponent;

#[function_component(App)]
pub fn app() -> Html {
    let selected_pf_algorithm = use_state(|| None);
    let selected_mg_algorithm = use_state(|| None);
    let reset_board = use_state(|| false);
    let reset_visited = use_state(|| false);

    let selected_pf_algorithm_clone = selected_pf_algorithm.clone();
    let on_find_path_clicked: Callback<PFAlgorithms> =
        Callback::from(move |algorithm: PFAlgorithms| {
            selected_pf_algorithm_clone.set(Some(algorithm));
        });

    let selected_mg_algorithm_clone = selected_mg_algorithm.clone();
    let on_generate_maze_clicked: Callback<MGAlgorithms> =
        Callback::from(move |algorithm: MGAlgorithms| {
            selected_mg_algorithm_clone.set(Some(algorithm));
        });

    let selected_mg_algorithm_clone = selected_mg_algorithm.clone();
    let completed_maze_generation: Callback<()> = Callback::from(move |_| {
        selected_mg_algorithm_clone.set(None);
    });

    let selected_pf_algorithm_clone = selected_pf_algorithm.clone();
    let completed_path_finding: Callback<()> = Callback::from(move |_| {
        selected_pf_algorithm_clone.set(None);
    });

    let reset_board_clone = reset_board.clone();
    let on_reset_board_clicked: Callback<()> = Callback::from(move |_| {
        reset_board_clone.set(true);
    });

    let reset_board_clone = reset_board.clone();
    let completed_reset_board: Callback<()> = Callback::from(move |_| {
        reset_board_clone.set(false);
    });

    let reset_visited_clone = reset_visited.clone();
    let on_reset_visited_clicked: Callback<()> = Callback::from(move |_| {
        reset_visited_clone.set(true);
    });

    let reset_visited_clone = reset_visited.clone();
    let completed_reset_visited: Callback<()> = Callback::from(move |_| {
        reset_visited_clone.set(false);
    });

    html! {
        <div>
            <AlgorithmSelectorComponent
                {on_find_path_clicked}
                {on_generate_maze_clicked}
                {on_reset_board_clicked}
                {on_reset_visited_clicked}
             />

            <MatrixComponent
                selected_mg_algorithm = { *selected_mg_algorithm }
                selected_pf_algorithm = { *selected_pf_algorithm }
                reset_board = { *reset_board }
                reset_visited = { *reset_visited }
                completed_maze_generation = { completed_maze_generation }
                completed_path_finding = { completed_path_finding }
                completed_reset_board = { completed_reset_board }
                completed_reset_visited = { completed_reset_visited }
            />
        </div>
    }
}
