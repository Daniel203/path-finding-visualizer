use super::cell::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix {
    pub x: u32,
    pub y: u32,
    pub matrix: Vec<Vec<Cell>>,
    pub start: Option<(u32, u32)>,
    pub end: Option<(u32, u32)>,
}

impl Matrix {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            matrix: vec![vec![Cell::Void; x as usize]; y as usize],
            start: None,
            end: None,
        }
    }

    pub fn add_start(&mut self, coords: (u32, u32)) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Start;
        self.start = Some(coords);
    }

    pub fn add_end(&mut self, coords: (u32, u32)) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::End;
        self.end = Some(coords);
    }

    pub fn toggle_cell(&mut self, coords: (u32, u32)) -> () {
        let cell_type = self.matrix[coords.1 as usize][coords.0 as usize].clone();

        match cell_type {
            Cell::Void => self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Wall,
            Cell::Wall => self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Void,
            Cell::Start => {
                self.start = None;
                self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Void;
            }
            Cell::End => {
                self.end = None;
                self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Void;
            }
        }
    }

    pub fn set_cell(&mut self, coords: (u32, u32), celltype: Cell) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = celltype;
    }
}
