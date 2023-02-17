use yewdux::prelude::*;

use crate::models::matrix::Matrix;

#[derive(Store, Default, PartialEq, Clone)]
pub struct MatrixState {
    pub matrix: Matrix,
    pub mouse_down: bool,
}
