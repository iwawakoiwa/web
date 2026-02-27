use pulldown_cmark::{Parser,Options,html};

pub fn to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut result = String::new();
    html::push_html(&mut result, parser);
    result
}