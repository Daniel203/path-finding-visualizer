use gloo_timers::callback::Timeout;
use yew::{function_component, html, Html};
use yewdux::prelude::{use_store, Dispatch};

use crate::algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms};
use crate::components::custom_components::cell_component::CellComponent;
use crate::components::store::algorithm_selector_state::AlgorithmSelectorState;
use crate::models::{cell::Cell, matrix::Matrix};

use crate::components::store::matrix_state::MatrixState;

#[function_component(MatrixComponent)]
pub fn matrix_component() -> Html {
    let (algorithm_selector_state, algorithm_selector_dispatch) =
        use_store::<AlgorithmSelectorState>();
    let (matrix_state, matrix_dispatch) = use_store::<MatrixState>();

    if algorithm_selector_state.generate_maze_clicked {
        if let Some(function) = algorithm_selector_state
            .selected_mg_algorithm
            .get_function()
        {
            function(&matrix_dispatch);
        }

        algorithm_selector_dispatch.reduce_mut(|state| {
            state.selected_mg_algorithm = MGAlgorithms::default();
            state.generate_maze_clicked = false;
        });
    }

    if algorithm_selector_state.find_path_clicked {
        if let Some(function) = algorithm_selector_state
            .selected_pf_algorithm
            .get_function()
        {
            function(&matrix_dispatch);
        }

        algorithm_selector_dispatch.reduce_mut(|state| {
            state.selected_pf_algorithm = PFAlgorithms::default();
            state.find_path_clicked = false;
        });
    }

    if algorithm_selector_dispatch.get().reset_board_clicked {
        matrix_dispatch.reduce_mut(|state| state.matrix.set_all_cells(Cell::UnVisited));
        algorithm_selector_dispatch.reduce_mut(|state| state.reset_board_clicked = false);
    }

    if algorithm_selector_dispatch.get().reset_visited_clicked {
        matrix_dispatch.reduce_mut(|state| {
            state
                .matrix
                .replace_cells(vec![Cell::Visited, Cell::Path], Cell::UnVisited)
        });
        algorithm_selector_dispatch.reduce_mut(|state| state.reset_visited_clicked = false);
    }

    html! {
        <div >
            <table>
            {
                matrix_state.matrix.matrix
                    .iter()
                    .enumerate()
                    .map( |(y, line)| {
                        html! {
                            <tr>
                            {   line.iter()
                                    .enumerate()
                                    .map(|(x, cell)| {
                                        return html! {
                                            <CellComponent
                                                x={x}
                                                y={y}
                                                cell_type={cell.clone()}
                                            />
                                        };
                                    })
                                    .collect::<Html>()
                            }
                            </tr>
                        }
                    })
                .collect::<Html>()
            }
            </table>
        </div>
    }
}

pub fn draw_matrix(matrix_dispatch: Dispatch<MatrixState>, matrix: Matrix) {
    Timeout::new(0, move || {
        matrix_dispatch.reduce_mut(|state| state.matrix = matrix);
    })
    .forget();
}
