use markdown::render;
use super::rouille;
pub fn preview_render_view(text:Option<String>)->rouille::Response{
    let text:String = text.unwrap_or(String::new());
    let mut out = Vec::new();
    use std::io::Write;
    render(&text).to_html(&mut out);
    rouille::Response::html(String::from_utf8(out).unwrap())
}