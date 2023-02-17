use std::ops::Deref;

use yew::{function_component, html, use_state, Callback, Html};

use crate::algorithms::maze_generation::MGAlgorithms;
use crate::algorithms::path_finding::PFAlgorithms;
use crate::components::algorithm_selector_component::AlgorithmSelectorComponent;
use crate::components::matrix_component::MatrixComponent;

#[derive(Default, Clone)]
struct State {
    selected_pf_algorithm: Option<PFAlgorithms>,
    selected_mg_algorithm: Option<MGAlgorithms>,
    reset_board: bool,
    reset_visited: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(State::default);

    let state_clone = state.clone();
    let set_pf_algorithm: Callback<Option<PFAlgorithms>> =
        Callback::from(move |algorithm: Option<PFAlgorithms>| {
            state_clone.set(State {
                selected_pf_algorithm: algorithm,
                ..state_clone.deref().clone()
            });
        });

    let state_clone = state.clone();
    let set_mg_algorithm: Callback<Option<MGAlgorithms>> =
        Callback::from(move |algorithm: Option<MGAlgorithms>| {
            state_clone.set(State {
                selected_mg_algorithm: algorithm,
                ..state_clone.deref().clone()
            });
        });

    let state_clone = state.clone();
    let set_reset_board: Callback<bool> = Callback::from(move |reset_board: bool| {
        state_clone.set(State {
            reset_board,
            ..state_clone.deref().clone()
        });
    });

    let state_clone = state.clone();
    let set_reset_visited: Callback<bool> = Callback::from(move |reset_visited: bool| {
        state_clone.set(State {
            reset_visited,
            ..state_clone.deref().clone()
        });
    });

    html! {
        <div>
            <AlgorithmSelectorComponent
                set_pf_algorithm = { set_pf_algorithm.clone() }
                set_mg_algorithm =  { set_mg_algorithm.clone() }
                set_reset_board =  { set_reset_board.clone() }
                set_reset_visited =  { set_reset_visited.clone() }
             />

            <MatrixComponent
                selected_mg_algorithm = { state.selected_mg_algorithm }
                selected_pf_algorithm = { state.selected_pf_algorithm }
                reset_board = { state.reset_board }
                reset_visited = { state.reset_visited }
                set_pf_algorithm = { set_pf_algorithm }
                set_mg_algorithm = { set_mg_algorithm }
                set_reset_board = { set_reset_board }
                set_reset_visited = { set_reset_visited }
            />
        </div>
    }
}
