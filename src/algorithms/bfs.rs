use std::collections::VecDeque;

use yew::UseStateHandle;

use crate::models::{cell::Cell, matrix::Matrix};

pub fn bfs<'a>(matrix_obj: UseStateHandle<Matrix>) -> Result<i32, &'a str> {
    let mut matrix = (*matrix_obj).clone();
    let start = matrix.start.unwrap();
    let end = matrix.end.unwrap();

    let mut queue = VecDeque::from([(0, start)]);

    let mut visited: Vec<Vec<Option<(isize, isize)>>> = Vec::new();
    let height = matrix.matrix.len();
    let width = matrix.matrix[0].len();
    initialize_visited(height, width, &mut visited);

    while queue.is_empty() == false {
        let (distance, coords) = queue.pop_front().unwrap();

        if coords == end {
            write_shortest_path(coords, start, &visited, &mut matrix);
            matrix_obj.set(matrix.clone());
            return Ok(distance);
        }

        handle_neighbours(&mut matrix, coords, &mut visited, &mut queue, distance);
    }

    return Err("Path not found");
}

fn handle_neighbours(
    matrix: &mut Matrix,
    coords: (isize, isize),
    visited: &mut Vec<Vec<Option<(isize, isize)>>>,
    queue: &mut VecDeque<(i32, (isize, isize))>,
    distance: i32,
) {
    for neighbour in get_neighbours(matrix.matrix.clone(), coords) {
        if (*visited)[neighbour.1 as usize][neighbour.0 as usize].is_none() {
            queue.push_back((distance + 1, neighbour));
            matrix.set_cell(coords, Cell::Seen);
            (*visited)[neighbour.1 as usize][neighbour.0 as usize] = Some((coords.1, coords.0));
        }
    }
}

fn get_neighbours(matrix: Vec<Vec<Cell>>, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = coords;
    let directions: Vec<(isize, isize)> = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

    let height = matrix.len() as isize;
    let width = matrix[0].len() as isize;

    let mut out: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_x, n_y) in directions {
        if n_x >= 0 && n_x < width && n_y >= 0 && n_y < height {
            if matrix[n_y as usize][n_x as usize] != Cell::Wall {
                out.push((n_x, n_y));
            }
        }
    }

    return out;
}

fn write_shortest_path(
    coords: (isize, isize),
    start: (isize, isize),
    visited: &Vec<Vec<Option<(isize, isize)>>>,
    matrix: &mut Matrix,
) {
    let (mut prev_x, mut prev_y) = coords;

    while (prev_x, prev_y) != start {
        let (py, px) = (*visited)[prev_y as usize][prev_x as usize].unwrap();
        matrix.set_cell((prev_x, prev_y), Cell::Path);

        prev_y = py;
        prev_x = px;
    }
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
