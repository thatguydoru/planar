use rinja::Template;

#[derive(Template)]
#[template(path = "board/index.html")]
pub struct BoardIndexTemplate {}
