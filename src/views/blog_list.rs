use dioxus::prelude::*;
use crate::Route;
use crate::views::markdown::to_html;

#[component]
pub fn BlogList() -> Element {
    let content = "# こんにちは\nこれは**Rust**製ブログです！";

    // MarkdownをHTMLに変換
    let html = to_html(content);

    rsx! {
        h1 { "ブログ一覧" }
        Link { to: Route::Blog { id: 0 }, "記事1" }
        Link { to: Route::Blog { id: 1 }, "記事2" }
        div {
            id: "blog",
            // 変換したHTMLをそのまま表示
            div { dangerous_inner_html: "{html}" }
        }
    }
}