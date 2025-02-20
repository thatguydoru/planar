use axum::response::Html;
use rinja::Template;

pub fn render_now(t: impl Template) -> Html<String> {
    Html(t.render().expect("must render"))
}
