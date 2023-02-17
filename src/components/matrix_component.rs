use gloo_timers::callback::Timeout;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, Html};
use yewdux::prelude::{use_store, Dispatch};

use crate::algorithms::{maze_generation, path_finding};
use crate::algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms};
use crate::components::store::algorithm_selector_state::AlgorithmSelectorState;
use crate::models::{cell::Cell, matrix::Matrix};

use super::store::matrix_state::MatrixState;

#[function_component(MatrixComponent)]
pub fn matrix_component() -> Html {
    let (algorithm_selector_state, algorithm_selector_dispatch) =
        use_store::<AlgorithmSelectorState>();
    let (matrix_state, matrix_dispatch) = use_store::<MatrixState>();

    if algorithm_selector_state.generate_maze_clicked {
        match algorithm_selector_state.selected_mg_algorithm {
            MGAlgorithms::NotSelected => {}
            MGAlgorithms::BinaryTree => {
                maze_generation::binary_tree::binary_tree(&matrix_dispatch);
            }
            MGAlgorithms::Dfs => {
                maze_generation::dfs::dfs(&matrix_dispatch);
            }
            MGAlgorithms::RecursiveDivision => {
                maze_generation::recursive_division::recursive_division(&matrix_dispatch);
            }
        };

        algorithm_selector_dispatch.reduce_mut(|state| {
            state.selected_mg_algorithm = MGAlgorithms::default();
            state.generate_maze_clicked = false;
        });
    }

    if algorithm_selector_state.find_path_clicked {
        match algorithm_selector_state.selected_pf_algorithm {
            PFAlgorithms::NotSelected => {}
            PFAlgorithms::Bfs => {
                path_finding::bfs::bfs(&matrix_dispatch);
            }
            PFAlgorithms::Dfs => {
                path_finding::dfs::dfs(&matrix_dispatch);
            }
            PFAlgorithms::AStar => {
                path_finding::a_star::a_star(&matrix_dispatch);
            }
            PFAlgorithms::AStarSearch => {
                path_finding::a_star_search::a_star_search(&matrix_dispatch);
            }
        };

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
        <div>
            <table>
            {
                matrix_state.matrix.matrix
                    .iter()
                    .enumerate()
                    .map( |(y, line)| {
                        html! {
                            <tr>{table_row(line, &matrix_dispatch, y)}</tr>
                        }
                    })
                .collect::<Html>()
            }
            </table>
        </div>
    }
}

fn table_row(line: &[Cell], matrix_dispatch: &Dispatch<MatrixState>, y: usize) -> Html {
    return line
        .iter()
        .enumerate()
        .map(|(x, cell)| table_cell(matrix_dispatch.clone(), cell, x, y))
        .collect::<Html>();
}

fn table_cell(matrix_dispatch: Dispatch<MatrixState>, cell: &Cell, x: usize, y: usize) -> Html {
    let onclick = matrix_dispatch.reduce_mut_callback(move |state| {
        state.matrix = handle_cell_clicked(state.matrix.clone(), x, y);
    });

    let onmouseenter = matrix_dispatch.reduce_mut_callback_with(move |state, _: MouseEvent| {
        if state.mouse_down {
            state.matrix = handle_cell_clicked(state.matrix.clone(), x, y);
        }
    });

    let onmouseup = matrix_dispatch.reduce_mut_callback_with(move |state, event: MouseEvent| {
        if event.button() == 0 {
            state.mouse_down = false;
        }
    });

    let onmousedown = matrix_dispatch.reduce_mut_callback_with(move |state, event: MouseEvent| {
        if event.button() == 0 {
            state.mouse_down = true;
        }
    });

    return html! {
        <td
            class={classes!(cell.class_name())}
            {onclick}
            {onmouseup}
            {onmousedown}
            {onmouseenter}
        ></td>
    };
}

fn handle_cell_clicked(matrix: Matrix, x: usize, y: usize) -> Matrix {
    let mut matrix = matrix;
    let coords = (x as isize, y as isize);

    if matrix.start.is_none() && matrix.get_cell(coords) != Some(Cell::End) {
        matrix.add_start(coords);
    } else if matrix.end.is_none() && matrix.get_cell(coords) != Some(Cell::Start) {
        matrix.add_end(coords);
    } else {
        matrix.toggle_cell(coords);
    }

    return matrix;
}

pub fn draw_matrix(matrix_dispatch: Dispatch<MatrixState>, matrix: Matrix) {
    Timeout::new(0, move || {
        matrix_dispatch.reduce_mut(|state| state.matrix = matrix);
    })
    .forget();
}
