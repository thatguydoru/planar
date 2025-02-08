use rinja::Template;

use crate::models::{Board, Column};

#[derive(Template)]
#[template(path = "board/index.html")]
pub struct BoardIndexTemplate {
    pub board: Board,
    pub columns: Vec<Column>,
}
