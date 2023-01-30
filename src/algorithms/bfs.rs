use gloo_timers::callback::Timeout;
use std::collections::VecDeque;

use yew::UseStateHandle;

use crate::models::{cell::Cell, matrix::Matrix};

pub fn bfs(matrix_obj: UseStateHandle<Matrix>) -> Option<i32> {
    let mut matrix = (*matrix_obj).clone();
    let start = matrix.start.unwrap();
    let end = matrix.end.unwrap();

    let mut queue = VecDeque::from([(0, start)]);

    let mut visited: Vec<Vec<Option<(isize, isize)>>> = Vec::new();
    let height = matrix.matrix.len();
    let width = matrix.matrix[0].len();
    initialize_visited(height, width, &mut visited);

    while !queue.is_empty() {
        let (distance, coords) = queue.pop_front().unwrap();

        if coords == end {
            write_shortest_path(coords, start, &visited, &mut matrix, &matrix_obj);
            return Some(distance);
        }

        handle_neighbours(
            matrix_obj.clone(),
            &mut matrix,
            coords,
            &mut visited,
            &mut queue,
            distance,
        );
    }

    return None;
}

fn handle_neighbours(
    matrix_obj: UseStateHandle<Matrix>,
    matrix: &mut Matrix,
    coords: (isize, isize),
    visited: &mut Vec<Vec<Option<(isize, isize)>>>,
    queue: &mut VecDeque<(i32, (isize, isize))>,
    distance: i32,
) {
    for neighbour in get_neighbours(matrix.matrix.clone(), coords) {
        if (*visited)[neighbour.1 as usize][neighbour.0 as usize].is_none() {
            queue.push_back((distance + 1, neighbour));
            (*visited)[neighbour.1 as usize][neighbour.0 as usize] = Some((coords.1, coords.0));

            matrix.set_cell(coords, Cell::Seen);
        }
    }

    render_new_matrix_state(matrix_obj.clone(), matrix.clone());
}

fn render_new_matrix_state(matrix_obj: UseStateHandle<Matrix>, matrix: Matrix) {
    Timeout::new(0, move || {
        matrix_obj.set(matrix.clone());
    })
    .forget();
}

fn get_neighbours(matrix: Vec<Vec<Cell>>, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = coords;
    let directions: Vec<(isize, isize)> = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

    let height = matrix.len() as isize;
    let width = matrix[0].len() as isize;

    let mut out: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_x, n_y) in directions {
        if n_x >= 0
            && n_x < width
            && n_y >= 0
            && n_y < height
            && matrix[n_y as usize][n_x as usize] != Cell::Wall
        {
            out.push((n_x, n_y));
        }
    }

    return out;
}

fn write_shortest_path(
    coords: (isize, isize),
    start: (isize, isize),
    visited: &Vec<Vec<Option<(isize, isize)>>>,
    matrix: &mut Matrix,
    matrix_obj: &UseStateHandle<Matrix>,
) {
    let (mut prev_x, mut prev_y) = coords;
    let mut path: Vec<(isize, isize)> = Vec::new();

    // get the shortest path
    while (prev_x, prev_y) != start {
        let (py, px) = (*visited)[prev_y as usize][prev_x as usize].unwrap();
        path.push((py, px));

        prev_y = py;
        prev_x = px;
    }

    // draw the path
    path.iter().rev().for_each(|(y, x)| {
        matrix.set_cell((*x, *y), Cell::Path);
        render_new_matrix_state(matrix_obj.clone(), matrix.clone());
    });
}

fn initialize_visited(height: usize, width: usize, visited: &mut Vec<Vec<Option<(isize, isize)>>>) {
    for _ in 0..height {
        let mut row = Vec::new();

        for _ in 0..width {
            row.push(None);
        }

        visited.push(row);
    }
}
