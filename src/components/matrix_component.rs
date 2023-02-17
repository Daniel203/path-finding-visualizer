use gloo_timers::callback::Timeout;
use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, use_state, Callback, Html, Properties, UseStateHandle,
};

use crate::algorithms::{maze_generation, path_finding};
use crate::algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms};
use crate::{
    constraints::{BOARD_HEIGHT, BOARD_WIDTH},
    models::{cell::Cell, matrix::Matrix},
};

#[derive(Properties, PartialEq, Clone)]
pub struct MatrixProps {
    pub selected_mg_algorithm: Option<MGAlgorithms>,
    pub selected_pf_algorithm: Option<PFAlgorithms>,
    pub reset_board: bool,
    pub reset_visited: bool,
    pub set_pf_algorithm: Callback<Option<PFAlgorithms>>,
    pub set_mg_algorithm: Callback<Option<MGAlgorithms>>,
    pub set_reset_board: Callback<bool>,
    pub set_reset_visited: Callback<bool>,
}

#[function_component(MatrixComponent)]
pub fn matrix_component(props: &MatrixProps) -> Html {
    let matrix_handle = use_state(|| Matrix::new(BOARD_WIDTH, BOARD_HEIGHT));
    let mouse_down = use_state(|| false);

    let props = props.clone();

    if let Some(algorithm) = props.selected_mg_algorithm {
        match algorithm {
            MGAlgorithms::NotSelected => {}
            MGAlgorithms::BinaryTree => {
                maze_generation::binary_tree::binary_tree(matrix_handle.clone());
            }
            MGAlgorithms::Dfs => {
                maze_generation::dfs::dfs(matrix_handle.clone());
            }
            MGAlgorithms::RecursiveDivision => {
                maze_generation::recursive_division::recursive_division(matrix_handle.clone());
            }
        };

        props.set_mg_algorithm.emit(None);
    }

    if let Some(algorithm) = props.selected_pf_algorithm {
        match algorithm {
            PFAlgorithms::NotSelected => {}
            PFAlgorithms::Bfs => {
                path_finding::bfs::bfs(matrix_handle.clone());
            }
            PFAlgorithms::Dfs => {
                path_finding::dfs::dfs(matrix_handle.clone());
            }
            PFAlgorithms::AStar => {
                path_finding::a_star::a_star(matrix_handle.clone());
            }
            PFAlgorithms::AStarSearch => {
                path_finding::a_star_search::a_star_search(matrix_handle.clone());
            }
        };

        props.set_pf_algorithm.emit(None);
    }

    if props.reset_board {
        let mut matrix = (*matrix_handle).clone();
        matrix.set_all_cells(Cell::UnVisited);
        matrix_handle.set(matrix);
        props.set_reset_board.emit(false);
    }

    if props.reset_visited {
        let mut matrix = (*matrix_handle).clone();
        matrix.replace_cells(vec![Cell::Visited, Cell::Path], Cell::UnVisited);
        matrix_handle.set(matrix);
        props.set_reset_visited.emit(false);
    }

    html! {
        <div>
            <table>
            {
                matrix_handle.matrix
                    .iter()
                    .enumerate()
                    .map(|(y, line)| {
                        html! {
                            <tr>{table_row(line, &matrix_handle, &mouse_down, y)}</tr>
                        }
                    })
                .collect::<Html>()
            }
            </table>
        </div>
    }
}

fn table_row(
    line: &[Cell],
    matrix_handle: &UseStateHandle<Matrix>,
    mouse_down: &UseStateHandle<bool>,
    y: usize,
) -> Html {
    return line
        .iter()
        .enumerate()
        .map(|(x, cell)| table_cell(matrix_handle, mouse_down, cell, x, y))
        .collect::<Html>();
}

fn table_cell(
    matrix_handle: &UseStateHandle<Matrix>,
    mouse_down: &UseStateHandle<bool>,
    cell: &Cell,
    x: usize,
    y: usize,
) -> Html {
    let onclick = {
        let matrix_handle = matrix_handle.clone();

        move |_| {
            let new_matrix = handle_cell_clicked(&matrix_handle, x, y);

            matrix_handle.set(new_matrix);
        }
    };

    let onmouseenter = {
        let mouse_down = mouse_down.clone();
        let matrix_handle = matrix_handle.clone();

        move |_| {
            if *mouse_down {
                let new_matrix = handle_cell_clicked(&matrix_handle, x, y);

                matrix_handle.set(new_matrix);
            }
        }
    };

    let onmouseup = {
        let mouse_down = mouse_down.clone();

        move |event: MouseEvent| {
            if event.button() == 0 {
                mouse_down.set(false);
            }
        }
    };

    let onmousedown = {
        let mouse_down = mouse_down.clone();

        move |event: MouseEvent| {
            if event.button() == 0 {
                mouse_down.set(true);
            }
        }
    };

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

fn handle_cell_clicked(matrix_handle: &UseStateHandle<Matrix>, x: usize, y: usize) -> Matrix {
    let mut new_matrix = (**matrix_handle).clone();
    let coords = (x as isize, y as isize);

    if new_matrix.start.is_none() && new_matrix.get_cell(coords) != Some(Cell::End) {
        new_matrix.add_start(coords);
    } else if new_matrix.end.is_none() && new_matrix.get_cell(coords) != Some(Cell::Start) {
        new_matrix.add_end(coords);
    } else {
        new_matrix.toggle_cell(coords);
    }

    return new_matrix;
}

pub fn draw_matrix(matrix_obj: UseStateHandle<Matrix>, matrix: Matrix) {
    Timeout::new(0, move || {
        matrix_obj.set(matrix.clone());
    })
    .forget();
}
