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
            matrix: vec![vec![Cell::UnSeen; x as usize]; y as usize],
            start: None,
            end: None,
        }
    }

    pub fn add_start(&mut self, coords: (isize, isize)) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Start;
        self.start = Some(coords);
    }

    pub fn add_end(&mut self, coords: (isize, isize)) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::End;
        self.end = Some(coords);
    }

    pub fn toggle_cell(&mut self, coords: (isize, isize)) -> () {
        let cell_type = self.matrix[coords.1 as usize][coords.0 as usize].clone();

        match cell_type {
            Cell::UnSeen => self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Wall,
            Cell::Wall => self.matrix[coords.1 as usize][coords.0 as usize] = Cell::UnSeen,
            Cell::Start => {
                self.start = None;
                self.matrix[coords.1 as usize][coords.0 as usize] = Cell::UnSeen;
            }
            Cell::End => {
                self.end = None;
                self.matrix[coords.1 as usize][coords.0 as usize] = Cell::UnSeen;
            }
            _ => (),
        }
    }

    pub fn set_cell(&mut self, coords: (isize, isize), celltype: Cell) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = celltype;
    }
}
