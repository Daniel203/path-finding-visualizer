use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, use_state, Callback, Html, Properties, UseStateHandle,
};

use crate::{
    algorithms::bfs::bfs,
    components::algorithm_selector_component::AlgorithmSelectorComponent,
    models::{cell::Cell, matrix::Matrix},
};

use super::algorithm_selector_component::PFAlgorithms;

#[function_component(MatrixComponent)]
pub fn matrix_component() -> Html {
    let matrix_handle = use_state(|| Matrix::new(40, 20));
    let mouse_down = use_state(|| false);

    let matrix_clone = matrix_handle.clone();

    let on_find_path_clicked: Callback<PFAlgorithms> =
        Callback::from(move |algorithm: PFAlgorithms| {
            //TODO: call the algorithm and draw on the frontend the matrix updated
            //if !matrix_clone.start.is_none() && !matrix_clone.end.is_none() {
            //match algorithm {
            //PFAlgorithms::BFS => {
            //bfs(matrix_clone.matrix, matrix_clone.start.unwrap(), matrix_clone.end.unwrap());
            //},
            //PFAlgorithms::DFS => todo!(),
            //}
            //}
        });

    html! {
        <div>
            <AlgorithmSelectorComponent {on_find_path_clicked} />
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
    line: &Vec<Cell>,
    matrix_handle: &UseStateHandle<Matrix>,
    mouse_down: &UseStateHandle<bool>,
    y: usize,
) -> Html {
    return line
        .iter()
        .enumerate()
        .map(|(x, cell)| table_cell(&matrix_handle, &mouse_down, cell, x, y))
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
