use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Void,
    Wall,
    Start,
    End,
}

impl Cell {
    pub fn class_name(&self) -> String {
        match self {
            Cell::Void => String::from("cell cell-void"),
            Cell::Wall => String::from("cell cell-wall"),
            Cell::Start => String::from("cell cell-start"),
            Cell::End => String::from("cell cell-end"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Void => write!(f, " "),
            Cell::Wall => write!(f, "W"),
            Cell::Start => write!(f, "S"),
            Cell::End => write!(f, "E"),
        }
    }
}
