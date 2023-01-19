use std::fmt::Display;

use gloo_console::log;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Void,
    Wall,
    Start,
    End,
}

impl Cell {
    fn class_name(&self) -> String {
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Matrix {
    x: u32,
    y: u32,
    matrix: Vec<Vec<Cell>>,
    start: Option<(u32, u32)>,
    end: Option<(u32, u32)>,
}

impl Matrix {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            matrix: vec![vec![Cell::Void; x as usize]; y as usize],
            start: None,
            end: None,
        }
    }

    fn add_start(&mut self, coords: (u32, u32)) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::Start;
        self.start = Some(coords);
    }

    fn add_end(&mut self, coords: (u32, u32)) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = Cell::End;
        self.end = Some(coords);
    }

    fn toggle_cell(&mut self, coords: (u32, u32)) -> () {
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

    fn set_cell(&mut self, coords: (u32, u32), celltype: Cell) -> () {
        self.matrix[coords.1 as usize][coords.0 as usize] = celltype;
    }
}

#[function_component(MatrixComponent)]
fn matrix_component() -> Html {
    let matrix_handle = use_state(|| Matrix::new(50, 30));
    let mouse_down = use_state(|| false);

    html! {
        <table>
        {
            matrix_handle.matrix
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    html! {
                        <tr>
                        {
                            line
                                .iter()
                                .enumerate()
                                .map(|(x, cell)| {
                                    let matrix_handle = matrix_handle.clone();
                                    let mouse_down = mouse_down.clone();

                                    let onclick = {
                                        let matrix_handle = matrix_handle.clone();

                                        move |_| {
                                            let mut new_matrix = (*matrix_handle).clone();
                                            let coords = (x as u32, y as u32);

                                            if new_matrix.start.is_none() {
                                                new_matrix.add_start(coords);
                                            } else if new_matrix.end.is_none() {
                                                new_matrix.add_end(coords);
                                            } else {
                                                new_matrix.toggle_cell(coords);
                                            }

                                            matrix_handle.set(new_matrix);
                                        }
                                    };

                                    let onmouseenter = {
                                        let mouse_down = mouse_down.clone();
                                        let matrix_handle = matrix_handle.clone();

                                        move |_| {
                                            if *mouse_down {
                                                let mut new_matrix = (*matrix_handle).clone();
                                                let coords = (x as u32, y as u32);

                                                if new_matrix.start.is_none() {
                                                    new_matrix.add_start(coords);
                                                } else if new_matrix.end.is_none() {
                                                    new_matrix.add_end(coords);
                                                } else {
                                                    new_matrix.toggle_cell(coords);
                                                }

                                                matrix_handle.set(new_matrix);
                                            }
                                        }
                                    };

                                    let onmouseup = {
                                        let mouse_down = mouse_down.clone();

                                        move |event: MouseEvent| {
                                            if event.button() == 0 {
                                                mouse_down.set(false);
                                            }
                                        }
                                    };

                                    let onmousedown = {
                                        let mouse_down = mouse_down.clone();

                                        move |event: MouseEvent| {
                                            if event.button() == 0{
                                                mouse_down.set(true);
                                            }
                                        }
                                    };

                                    return html!{
                                        <td
                                            class={classes!(cell.class_name())}
                                            {onclick}
                                            {onmouseup}
                                            {onmousedown}
                                            {onmouseenter}
                                        ></td>
                                    };
                                })
                                .collect::<Html>()
                        }
                        </tr>
                    }
                })
            .collect::<Html>()
        }
        </table>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <MatrixComponent />
        </div>
    }
}

fn main() {
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
