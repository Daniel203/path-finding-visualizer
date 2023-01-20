use std::collections::{HashSet, VecDeque};

use crate::models::cell::Cell;

pub fn bfs(matrix: Vec<Vec<Cell>>, start: (isize, isize), end: (isize, isize)) -> i32 {
    let mut matrix = matrix;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue = VecDeque::from([(0, start)]);

    while queue.is_empty() == false {
        let (distance, coords) = queue.pop_front().unwrap();
        let coords = coords;

        if coords == end {
            return distance;
        }

        if visited.contains(&coords) == false {
            visited.insert(coords);

            for neighbour in get_neighbours(&matrix, coords) {
                if visited.contains(&neighbour) == false {
                    queue.push_back((distance + 1, neighbour));
                    matrix[coords.0 as usize][coords.1 as usize] = Cell::Seen;
                }
            }
        }
    }

    return -1;
}

fn get_neighbours(matrix: &Vec<Vec<Cell>>, coords: (isize, isize)) -> Vec<(isize, isize)> {
    let (row, col) = coords;

    let height = matrix.len() as isize;
    let width = matrix[0].len() as isize;

    let directions: Vec<(isize, isize)> = vec![
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ];
    let mut out: Vec<(isize, isize)> = Vec::new();

    // coordinates of the neighbour
    for (n_row, n_col) in directions {
        if n_row >= 0 && n_row < height && n_col >= 0 && n_col < width {
            if matrix[n_row as usize][n_col as usize] != Cell::Wall {
                out.push((n_row, n_col));
            }
        }
    }

    return out;
}
