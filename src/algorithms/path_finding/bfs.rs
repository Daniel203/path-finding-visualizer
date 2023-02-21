use std::collections::VecDeque;

use yewdux::prelude::Dispatch;

use crate::components::custom_components::matrix_component::draw_matrix;
use crate::{
    components::store::matrix_state::MatrixState,
    models::{cell::Cell, matrix::Matrix},
};

pub fn bfs(matrix_dispatch: &Dispatch<MatrixState>) -> Option<i32> {
    let mut matrix = matrix_dispatch.get().matrix.clone();
    let start = matrix.start.unwrap();
    let end = matrix.end.unwrap();

    let mut queue = VecDeque::from([(0, start)]);

    let mut visited: Vec<Vec<Option<(isize, isize)>>> = Vec::new();
    let height = matrix.clone().height;
    let width = matrix.clone().width;
    initialize_visited(height as usize, width as usize, &mut visited);

    while !queue.is_empty() {
        let (distance, coords) = queue.pop_front().unwrap();

        if coords == end {
            write_shortest_path(coords, start, &visited, &mut matrix, matrix_dispatch);
            return Some(distance);
        }

        for neighbour in get_neighbours(&matrix, coords) {
            if visited[neighbour.1 as usize][neighbour.0 as usize].is_none() {
                queue.push_back((distance + 1, neighbour));
                visited[neighbour.1 as usize][neighbour.0 as usize] = Some(coords);

                if neighbour != start && neighbour != end {
                    matrix.set_cell(neighbour, Cell::Visited);
                }
            }
        }

        draw_matrix(matrix_dispatch.clone(), matrix.clone());
    }

    return None;
}

fn get_neighbours(matrix: &Matrix, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let width = matrix.clone().width;
    let height = matrix.clone().height;

    let (x, y) = coords;
    let directions: Vec<(isize, isize)> = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

    let mut neighbours: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_x, n_y) in directions {
        if n_x >= 0
            && n_x < width
            && n_y >= 0
            && n_y < height
            && matrix.get_cell((n_x, n_y)) != Some(Cell::Wall)
        {
            neighbours.push((n_x, n_y));
        }
    }

    return neighbours;
}

fn write_shortest_path(
    coords: (isize, isize),
    start: (isize, isize),
    visited: &[Vec<Option<(isize, isize)>>],
    matrix: &mut Matrix,
    matrix_dispatch: &Dispatch<MatrixState>,
) {
    let (mut prev_x, mut prev_y) = coords;
    let mut path: Vec<(isize, isize)> = Vec::new();

    // get the shortest path
    while (prev_x, prev_y) != start {
        let (px, py) = visited[prev_y as usize][prev_x as usize].unwrap();

        if (px, py) != start {
            path.push((px, py));
        }

        prev_y = py;
        prev_x = px;
    }

    // draw the path
    path.iter().rev().for_each(|(x, y)| {
        matrix.set_cell((*x, *y), Cell::Path);
        draw_matrix(matrix_dispatch.clone(), matrix.clone());
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
