use gloo_timers::callback::Timeout;
use rand::{seq::SliceRandom, thread_rng};
use yew::UseStateHandle;

use crate::models::{cell::Cell, matrix::Matrix};

pub fn binary_tree(matrix_obj: UseStateHandle<Matrix>) {
    let mut matrix = (*matrix_obj).clone();

    let mut y: isize = 0;
    while y < matrix.height() as isize {
        let mut x: isize = 0;

        while x < matrix.width() as isize {
            matrix.set_cell((x, y), Cell::Wall);
            let mut neighbours: Vec<(isize, isize)> = Vec::new();

            // get west neighbour
            let west_neighbour = get_neighbour_in_direction(&mut matrix, (x, y), (-2, 0));
            if let Some(neighbour) = west_neighbour {
                neighbours.push(neighbour);
            }

            // get north neighbour
            let north_neighbour = get_neighbour_in_direction(&mut matrix, (x, y), (0, -2));
            if let Some(neighbour) = north_neighbour {
                neighbours.push(neighbour);
            }

            // if there's a neighbour then connect it with wall
            if !neighbours.is_empty() {
                let neighbour = select_random_neighbour(&mut matrix, &mut neighbours);
                connect_cells(neighbour, &mut matrix, x, y);
            }

            render_new_matrix_state(matrix_obj.clone(), matrix.clone());

            x += 2;
        }

        y += 2;
    }

    render_new_matrix_state(matrix_obj, matrix);
}

fn get_neighbour_in_direction(
    matrix: &mut Matrix,
    curr_position: (isize, isize),
    direction: (isize, isize),
) -> Option<(isize, isize)> {
    let neighbour: (isize, isize) = (curr_position.0 + direction.0, curr_position.1 + direction.1);

    if matrix.is_valid_coords(neighbour) && matrix.get_cell(neighbour) == Some(Cell::Wall) {
        return Some(neighbour);
    }

    return None;
}

fn select_random_neighbour(
    matrix: &mut Matrix,
    neighbours: &mut Vec<(isize, isize)>,
) -> (isize, isize) {
    neighbours.shuffle(&mut thread_rng());

    let neighbour = neighbours.first().unwrap();
    matrix.set_cell(*neighbour, Cell::Wall);

    *neighbour
}

fn connect_cells(neighbour: (isize, isize), matrix: &mut Matrix, x: isize, y: isize) {
    let mut connection: (isize, isize) = (-1, -1);

    if neighbour.0 == x {
        connection = (x, y - 1);
    } else if neighbour.1 == y {
        connection = (x - 1, y);
    }

    if matrix.get_cell(connection) == Some(Cell::UnVisited) {
        matrix.set_cell(connection, Cell::Wall);
    }
}

fn render_new_matrix_state(matrix_obj: UseStateHandle<Matrix>, matrix: Matrix) {
    Timeout::new(0, move || {
        matrix_obj.set(matrix.clone());
    })
    .forget();
}