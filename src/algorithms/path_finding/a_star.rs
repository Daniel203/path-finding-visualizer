use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use yewdux::prelude::Dispatch;

use crate::{components::matrix_component::draw_matrix, models::matrix::Matrix};
use crate::{components::store::matrix_state::MatrixState, models::cell::Cell};

#[derive(Debug, Clone, Copy)]
struct Node {
    coords: (isize, isize),
    cost: f64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.cost.eq(&other.cost);
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.cost.partial_cmp(&self.cost).unwrap();
    }
}

pub fn a_star(matrix_dispatch: &Dispatch<MatrixState>) -> Option<i32> {
    let mut matrix = matrix_dispatch.get().matrix.clone();
    let start = matrix.start.unwrap();
    let end = matrix.end.unwrap();

    let mut frontier = BinaryHeap::<Node>::new();
    frontier.push(Node {
        coords: start,
        cost: manhattan_distance(start, end),
    });

    let mut came_from = HashMap::<(isize, isize), Option<(isize, isize)>>::new();
    let mut cost_so_far = HashMap::<(isize, isize), f64>::new();

    came_from.insert(start, None);
    cost_so_far.insert(start, 0.0);

    while !frontier.is_empty() {
        let curr = frontier.pop().unwrap();
        let coords = curr.coords;

        if coords == end {
            write_shortest_path(came_from, start, end, &mut matrix, matrix_dispatch);
            return Some(curr.cost as i32);
        }

        for neighbour in get_neighbours(&matrix, coords) {
            let new_cost = *cost_so_far.get(&coords).unwrap() + 1.0;

            if !cost_so_far.contains_key(&neighbour)
                || new_cost < *cost_so_far.entry(neighbour).or_insert(f64::INFINITY)
            {
                cost_so_far.insert(neighbour, new_cost);
                frontier.push(Node {
                    coords: neighbour,
                    cost: new_cost,
                });
                came_from.insert(neighbour, Some(coords));

                if neighbour != start && neighbour != end {
                    matrix.set_cell(neighbour, Cell::Visited);
                    draw_matrix(matrix_dispatch.clone(), matrix.clone());
                }
            }
        }
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

fn manhattan_distance(start: (isize, isize), end: (isize, isize)) -> f64 {
    return (isize::abs_diff(start.0, end.0) + isize::abs_diff(start.1, end.1)) as f64;
}

fn write_shortest_path(
    came_from: HashMap<(isize, isize), Option<(isize, isize)>>,
    start: (isize, isize),
    end: (isize, isize),
    matrix: &mut Matrix,
    matrix_dispatch: &Dispatch<MatrixState>,
) {
    let mut curr = (*came_from.get(&end).unwrap()).unwrap();
    let mut path = Vec::new();

    while curr != start {
        path.push(curr);
        curr = (*came_from.get(&curr).unwrap()).unwrap();
    }

    for coords in path.iter().rev() {
        matrix.set_cell(*coords, Cell::Path);
        draw_matrix(matrix_dispatch.clone(), matrix.clone());
    }
}
