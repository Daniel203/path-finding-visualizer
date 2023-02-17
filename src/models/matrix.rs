use crate::constraints;

use super::cell::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix {
    pub width: isize,
    pub height: isize,
    pub matrix: Vec<Vec<Cell>>,
    pub start: Option<(isize, isize)>,
    pub end: Option<(isize, isize)>,
}

impl Default for Matrix {
    fn default() -> Self {
        return Self::new(constraints::BOARD_WIDTH, constraints::BOARD_HEIGHT);
    }
}

impl Matrix {
    pub fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            matrix: vec![vec![Cell::UnVisited; width as usize]; height as usize],
            start: None,
            end: None,
        }
    }

    pub fn add_start(&mut self, coords: (isize, isize)) {
        if self.matrix[coords.1 as usize][coords.0 as usize] == Cell::End {
            self.end = None
        }
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Start;
        self.start = Some(coords);
    }

    pub fn add_end(&mut self, coords: (isize, isize)) {
        if self.matrix[coords.1 as usize][coords.0 as usize] == Cell::Start {
            self.start = None
        }
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::End;
        self.end = Some(coords);
    }

    pub fn toggle_cell(&mut self, coords: (isize, isize)) {
        let cell_type = self.matrix[coords.1 as usize][coords.0 as usize].clone();

        match cell_type {
            Cell::UnVisited => self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Wall,
            Cell::Wall => self.matrix[coords.1 as usize][coords.0 as usize] = Cell::UnVisited,
            Cell::Start => {
                self.start = None;
                self.matrix[coords.1 as usize][coords.0 as usize] = Cell::UnVisited;
            }
            Cell::End => {
                self.end = None;
                self.matrix[coords.1 as usize][coords.0 as usize] = Cell::UnVisited;
            }
            _ => (),
        }
    }

    pub fn get_cell(&self, coords: (isize, isize)) -> Option<Cell> {
        if self.is_valid_coords(coords) {
            return Some(self.matrix[coords.1 as usize][coords.0 as usize].clone());
        } else {
            return None;
        }
    }

    pub fn set_cell(&mut self, coords: (isize, isize), celltype: Cell) {
        if self.is_valid_coords(coords) {
            self.matrix[coords.1 as usize][coords.0 as usize] = celltype;
        }
    }

    pub fn set_all_cells(&mut self, celltype: Cell) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_cell((x, y), celltype.clone());
            }
        }

        // reset the start and end because they have been overwritten
        self.start = None;
        self.end = None;
    }

    pub fn is_valid_coords(&self, coords: (isize, isize)) -> bool {
        return coords.0 >= 0 && coords.1 >= 0 && coords.0 < self.width && coords.1 < self.height;
    }

    pub fn replace_cells(&mut self, cell_to_replace: Vec<Cell>, replace_with: Cell) {
        for (y, line) in self.clone().matrix.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell_to_replace.contains(cell) {
                    self.set_cell((x as isize, y as isize), replace_with.clone());
                }
            }
        }
    }
}
