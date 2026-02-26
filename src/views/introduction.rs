use dioxus::prelude::*;

#[component]
pub fn Introduction() -> Element {
    rsx! {
        h1 { "Aboutページ" }
        p { "ここに内容を書く" }
    }
}