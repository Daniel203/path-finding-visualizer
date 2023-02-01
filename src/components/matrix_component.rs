use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Html, UseStateHandle};

use crate::{
    algorithms::bfs::bfs,
    algorithms::dfs::dfs,
    components::algorithm_selector_component::AlgorithmSelectorComponent,
    constraints::{BOARD_HEIGHT, BOARD_WIDTH},
    models::{cell::Cell, matrix::Matrix},
};

use super::algorithm_selector_component::PFAlgorithms;

#[function_component(MatrixComponent)]
pub fn matrix_component() -> Html {
    let matrix_handle = use_state(|| Matrix::new(BOARD_WIDTH, BOARD_HEIGHT));
    let mouse_down = use_state(|| false);

    let matrix_for_find_path = matrix_handle.clone();
    let on_find_path_clicked: Callback<PFAlgorithms> =
        Callback::from(move |algorithm: PFAlgorithms| {
            if matrix_for_find_path.start.is_some() && matrix_for_find_path.end.is_some() {
                match algorithm {
                    PFAlgorithms::NotSelected => (),
                    PFAlgorithms::BFS => {
                        bfs(matrix_for_find_path.clone());
                    }
                    PFAlgorithms::DFS => {
                        dfs(matrix_for_find_path.clone());
                    }
                }
            }
        });

    let matrix_for_reset_board = matrix_handle.clone();
    let on_reset_board_clicked: Callback<()> = Callback::from(move |_| {
        let new_matrix = Matrix::new(BOARD_WIDTH, BOARD_HEIGHT);
        matrix_for_reset_board.set(new_matrix);
    });

    html! {
        <div>
            <AlgorithmSelectorComponent {on_find_path_clicked} {on_reset_board_clicked} />
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

    if new_matrix.start.is_none() {
        new_matrix.add_start(coords);
    } else if new_matrix.end.is_none() {
        new_matrix.add_end(coords);
    } else {
        new_matrix.toggle_cell(coords);
    }

    return new_matrix;
}
