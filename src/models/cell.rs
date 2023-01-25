use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    UnSeen,
    Seen,
    Path,
    Wall,
    Start,
    End,
}

impl Cell {
    pub fn class_name(&self) -> String {
        match self {
            Cell::UnSeen => String::from("cell cell-unseen"),
            Cell::Seen => String::from("cell cell-seen"),
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
            Cell::UnSeen => write!(f, "U"),
            Cell::Seen => write!(f, "V"),
            Cell::Path => write!(f, "P"),
            Cell::Wall => write!(f, "W"),
            Cell::Start => write!(f, "S"),
            Cell::End => write!(f, "E"),
        }
    }
}