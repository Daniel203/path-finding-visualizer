use rand::seq::SliceRandom;
use yewdux::prelude::Dispatch;

use crate::{
    components::{matrix_component::draw_matrix, store::matrix_state::MatrixState},
    models::{cell::Cell, matrix::Matrix},
};

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Vertical,
    Horizontal,
}

pub fn recursive_division(matrix_dispatch: &Dispatch<MatrixState>) {
    let mut matrix = matrix_dispatch.get().matrix.clone();
    matrix.set_all_cells(Cell::UnVisited);

    let width = matrix.width;
    let height = matrix.height;

    draw_surrounding_walls(matrix_dispatch.clone(), &mut matrix);

    divide(
        matrix_dispatch.clone(),
        &mut matrix,
        2,
        2,
        width - 3,
        height - 3,
        Orientation::Horizontal,
    );

    draw_matrix(matrix_dispatch.clone(), matrix);
}

fn draw_surrounding_walls(matrix_dispatch: Dispatch<MatrixState>, matrix: &mut Matrix) {
    let width = matrix.width;
    let height = matrix.height;

    // vertical walls
    for y in 0..height {
        matrix.set_cell((0, y), Cell::Wall);
        matrix.set_cell((width - 1, y), Cell::Wall);
        draw_matrix(matrix_dispatch.clone(), matrix.clone());
    }

    // horizontal walls
    for x in 0..width {
        matrix.set_cell((x, 0), Cell::Wall);
        matrix.set_cell((x, height - 1), Cell::Wall);
        draw_matrix(matrix_dispatch.clone(), matrix.clone());
    }
}

fn divide(
    matrix_dispatch: Dispatch<MatrixState>,
    matrix: &mut Matrix,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    orientation: Orientation,
) {
    if max_y < min_y || max_x < min_x {
        return;
    }

    let mut rng = rand::thread_rng();

    match orientation {
        Orientation::Vertical => {
            let curr_row = *(min_y - 1..=max_y - 1)
                .step_by(2)
                .collect::<Vec<isize>>()
                .choose(&mut rng)
                .unwrap();

            let curr_col = *(min_x..=max_x)
                .step_by(2)
                .collect::<Vec<isize>>()
                .choose(&mut rng)
                .unwrap();

            for y in min_y - 1..=max_y + 1 {
                matrix.set_cell((curr_col, y), Cell::Wall);
            }

            matrix.set_cell((curr_col, curr_row), Cell::UnVisited);
            draw_matrix(matrix_dispatch.clone(), matrix.clone());

            if curr_col - min_x - 2 < max_y - min_y {
                divide(
                    matrix_dispatch.clone(),
                    matrix,
                    min_x,
                    min_y,
                    curr_col - 2,
                    max_y,
                    Orientation::Horizontal,
                );
            } else {
                divide(
                    matrix_dispatch.clone(),
                    matrix,
                    min_x,
                    min_y,
                    curr_col - 2,
                    max_y,
                    orientation,
                );
            }

            if max_x - curr_col + 2 < max_y - min_y {
                divide(
                    matrix_dispatch,
                    matrix,
                    curr_col + 2,
                    min_y,
                    max_x,
                    max_y,
                    Orientation::Horizontal,
                );
            } else {
                divide(
                    matrix_dispatch,
                    matrix,
                    curr_col + 2,
                    min_y,
                    max_x,
                    max_y,
                    orientation,
                );
            }
        }
        Orientation::Horizontal => {
            let curr_row = *(min_y..=max_y)
                .step_by(2)
                .collect::<Vec<isize>>()
                .choose(&mut rng)
                .unwrap();

            let curr_col = *(min_x - 1..=max_x - 1)
                .step_by(2)
                .collect::<Vec<isize>>()
                .choose(&mut rng)
                .unwrap();

            for x in min_x - 1..=max_x + 1 {
                matrix.set_cell((x, curr_row), Cell::Wall);
            }

            matrix.set_cell((curr_col, curr_row), Cell::UnVisited);
            draw_matrix(matrix_dispatch.clone(), matrix.clone());

            if curr_row - min_y - 2 > max_x - max_y {
                divide(
                    matrix_dispatch.clone(),
                    matrix,
                    min_x,
                    min_y,
                    max_x,
                    curr_row - 2,
                    orientation,
                );
            } else {
                divide(
                    matrix_dispatch.clone(),
                    matrix,
                    min_x,
                    min_y,
                    max_x,
                    curr_row - 2,
                    Orientation::Vertical,
                );
            }

            if max_y - curr_row + 2 > max_x - min_x {
                divide(
                    matrix_dispatch,
                    matrix,
                    min_x,
                    curr_row + 2,
                    max_x,
                    max_y,
                    orientation,
                );
            } else {
                divide(
                    matrix_dispatch,
                    matrix,
                    min_x,
                    curr_row + 2,
                    max_x,
                    max_y,
                    Orientation::Vertical,
                );
            }
        }
    }
}
