use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    UnVisited,
    Visited,
    Path,
    Wall,
    Start,
    End,
}

impl Cell {
    pub fn class_name(&self) -> String {
        match self {
            Cell::UnVisited => String::from("cell cell-unvisited"),
            Cell::Visited => String::from("cell cell-visited"),
            Cell::Path => String::from("cell cell-path"),
            Cell::Wall => String::from("cell cell-wall"),
            Cell::Start => String::from("cell cell-start"),
            Cell::End => String::from("cell cell-end"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::UnVisited => write!(f, "U"),
            Cell::Visited => write!(f, "V"),
            Cell::Path => write!(f, "P"),
            Cell::Wall => write!(f, "W"),
            Cell::Start => write!(f, "S"),
            Cell::End => write!(f, "E"),
        }
    }
}
