use std::collections::VecDeque;

use gloo_timers::callback::Timeout;
use rand::{seq::SliceRandom, thread_rng, Rng};
use yew::UseStateHandle;

use crate::models::{cell::Cell, matrix::Matrix};

pub fn dfs(matrix_obj: UseStateHandle<Matrix>) {
    let mut matrix = (*matrix_obj).clone();
    let mut queue: VecDeque<(isize, isize)>;
    let mut visited: Vec<(isize, isize)> = Vec::new();

    matrix.set_all_cells(Cell::Wall);

    let start = get_random_cell(&mut matrix);
    queue = VecDeque::from([start]);

    while !queue.is_empty() {
        let curr = queue.pop_back().unwrap();
        visited.push(curr);

        let mut neighbours = get_neighbours(&matrix, curr);

        if !neighbours.is_empty() {
            for neighbour in neighbours.iter() {
                let connection = (
                    curr.0 + isize::signum(neighbour.0 - curr.0),
                    curr.1 + isize::signum(neighbour.1 - curr.1),
                );

                matrix.set_cell(*neighbour, Cell::UnVisited);
                matrix.set_cell(connection, Cell::UnVisited);
            }

            neighbours.shuffle(&mut thread_rng());

            for neighbour in neighbours.iter() {
                queue.push_back(*neighbour);
            }

            render_new_matrix_state(matrix_obj.clone(), matrix.clone());
        }
    }

    render_new_matrix_state(matrix_obj, matrix);
}

fn get_random_cell(matrix: &mut Matrix) -> (isize, isize) {
    let mut coords = (-1, -1);
    let mut rng = rand::thread_rng();

    while !matrix.is_valid_coords(coords) || matrix.get_cell(coords) != Some(Cell::Wall) {
        let x = rng.gen_range(0..matrix.width);
        let y = rng.gen_range(0..matrix.height);
        coords = (x, y)
    }

    return coords;
}

fn get_neighbours(matrix: &Matrix, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let width = matrix.clone().width;
    let height = matrix.clone().height;

    let (x, y) = coords;
    let directions: Vec<(isize, isize)> = vec![(x - 2, y), (x + 2, y), (x, y + 2), (x, y - 2)];

    let mut neighbours: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_x, n_y) in directions {
        if n_x >= 0
            && n_x < width
            && n_y >= 0
            && n_y < height
            && matrix.get_cell((n_x, n_y)) != Some(Cell::UnVisited)
            && matrix.get_cell((n_x, n_y)) != Some(Cell::Start)
            && matrix.get_cell((n_x, n_y)) != Some(Cell::End)
        {
            neighbours.push((n_x, n_y));
        }
    }

    return neighbours;
}

fn render_new_matrix_state(matrix_obj: UseStateHandle<Matrix>, matrix: Matrix) {
    Timeout::new(0, move || {
        matrix_obj.set(matrix.clone());
    })
    .forget();
}
