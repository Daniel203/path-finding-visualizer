use std::collections::{HashSet, VecDeque};

use gloo_console::log;
use yew::UseStateHandle;

use crate::models::{cell::Cell, matrix::Matrix};

pub fn bfs(matrix_obj: UseStateHandle<Matrix>) -> i32 {
    let mut matrix = (*matrix_obj).clone();
    let start = matrix.start.unwrap();
    let end = matrix.end.unwrap();

    let height = matrix.matrix.len();
    let width = matrix.matrix[0].len();

    let mut visited_: Vec<Vec<Option<(isize, isize)>>> = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(None);
        }
        visited_.push(row);
    }
    visited_[start.1 as usize][start.0 as usize] = Some((start.1, start.0));

    //let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue = VecDeque::from([(0, start)]);

    while queue.is_empty() == false {
        let (distance, coords) = queue.pop_front().unwrap();
        let coords = coords;

        if coords == end {
            log!("fineeeeee");
            matrix_obj.set(matrix.clone());
            return distance;
        }

        //if visited.contains(&coords) == false {
        //visited.insert(coords);

        //for neighbour in get_neighbours(matrix.matrix.clone(), coords) {
        //if visited.contains(&neighbour) == false {
        //queue.push_back((distance + 1, neighbour));
        //matrix.set_cell(coords, Cell::Seen);

        ////matrix_obj.set(matrix.clone());
        //}
        //}
        //}

        for neighbour in get_neighbours(matrix.matrix.clone(), coords) {
            log!("2");
            if visited_[neighbour.1 as usize][neighbour.0 as usize].is_none() {
                log!("3");
                log!("hey ciao");
                queue.push_back((distance + 1, neighbour));
                matrix.set_cell(coords, Cell::Seen);

                visited_[neighbour.0 as usize][neighbour.1 as usize] =
                    Some((neighbour.1, neighbour.0));
            }
        }
    }

    return -1;
}

fn get_neighbours(matrix: Vec<Vec<Cell>>, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = coords;

    let height = matrix.len() as isize;
    let width = matrix[0].len() as isize;

    let directions: Vec<(isize, isize)> = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

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