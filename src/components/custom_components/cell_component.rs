use web_sys::MouseEvent;
use yew::{function_component, html, Html, Properties};
use yewdux::prelude::use_store;

use crate::{
    components::store::matrix_state::MatrixState,
    models::{cell::Cell, matrix::Matrix},
};

#[derive(Properties, PartialEq, Clone)]
pub struct CellProps {
    pub x: usize,
    pub y: usize,
    pub cell_type: Option<Cell>,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &CellProps) -> Html {
    let (_, matrix_dispatch) = use_store::<MatrixState>();
    let props = props.clone();

    let onclick = matrix_dispatch.reduce_mut_callback(move |state| {
        state.matrix = handle_cell_clicked(state.matrix.clone(), props.x, props.y);
    });

    let onmouseenter = matrix_dispatch.reduce_mut_callback_with(move |state, _: MouseEvent| {
        if state.mouse_down {
            state.matrix = handle_cell_clicked(state.matrix.clone(), props.x, props.y);
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
            class={props.cell_type.unwrap_or_default().class_name()}
            {onclick}
            {onmouseenter}
            {onmousedown}
            {onmouseup}
        />
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
