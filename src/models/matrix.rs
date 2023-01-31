use super::cell::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix {
    pub x: isize,
    pub y: isize,
    pub matrix: Vec<Vec<Cell>>,
    pub start: Option<(isize, isize)>,
    pub end: Option<(isize, isize)>,
}

impl Matrix {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            matrix: vec![vec![Cell::UnVisited; x as usize]; y as usize],
            start: None,
            end: None,
        }
    }

    pub fn add_start(&mut self, coords: (isize, isize)) {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Start;
        self.start = Some(coords);
    }

    pub fn add_end(&mut self, coords: (isize, isize)) {
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

    fn is_valid_coords(&self, coords: (isize, isize)) -> bool {
        let height = self.matrix.len() as isize;
        let width = self.matrix[0].len() as isize;

        return coords.0 >= 0 && coords.1 >= 0 && coords.0 < width && coords.1 < height;
    }

    pub fn height(self) -> usize {
        return self.matrix.len();
    }

    pub fn width(&self) -> usize {
        return self.matrix[0].len();
    }
}
