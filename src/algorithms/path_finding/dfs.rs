use std::collections::VecDeque;

use yew::UseStateHandle;

use crate::{
    components::matrix_component::draw_matrix,
    models::{cell::Cell, matrix::Matrix},
};

pub fn dfs(matrix_obj: UseStateHandle<Matrix>) -> Option<i32> {
    let mut matrix = (*matrix_obj).clone();
    let start = matrix.start.unwrap();
    let end = matrix.end.unwrap();

    let mut queue = VecDeque::from([(0, start, start)]);

    let mut visited: Vec<Vec<Option<(isize, isize)>>> = Vec::new();
    let height = matrix.clone().height;
    let width = matrix.clone().width;
    initialize_visited(height as usize, width as usize, &mut visited);

    while !queue.is_empty() {
        let (distance, curr_coords, prev_coords) = queue.pop_back().unwrap();

        visited[prev_coords.1 as usize][prev_coords.0 as usize] = Some(curr_coords);

        if curr_coords == end {
            write_shortest_path(start, end, &visited, &mut matrix, &matrix_obj);
            return Some(distance);
        }

        if curr_coords != start {
            matrix.set_cell(curr_coords, Cell::Visited);
        }

        draw_matrix(matrix_obj.clone(), matrix.clone());

        for neighbour in get_neighbours(&matrix, curr_coords) {
            if visited[neighbour.1 as usize][neighbour.0 as usize].is_none() {
                queue.push_back((distance + 1, neighbour, curr_coords));
            }
        }
    }

    return None;
}

fn get_neighbours(matrix: &Matrix, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let width = matrix.clone().width;
    let height = matrix.clone().height;

    let (x, y) = coords;
    let directions: Vec<(isize, isize)> = vec![(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)];

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
    start: (isize, isize),
    end: (isize, isize),
    visited: &[Vec<Option<(isize, isize)>>],
    matrix: &mut Matrix,
    matrix_obj: &UseStateHandle<Matrix>,
) {
    let (mut next_x, mut next_y) = visited[start.1 as usize][start.0 as usize].unwrap();

    while (next_x, next_y) != end {
        matrix.set_cell((next_x, next_y), Cell::Path);
        draw_matrix(matrix_obj.clone(), matrix.clone());

        let (nx, ny) = visited[next_y as usize][next_x as usize].unwrap();

        next_y = ny;
        next_x = nx;
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
