use dioxus::prelude::*;

const INTRODUCTION_CSS: Asset = asset!("/assets/styling/introduction.css");
const MYICON_WEBP: Asset = asset!("assets/image/myicon.webp");

#[component]
pub fn introduction() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: INTRODUCTION_CSS }
        div {
            class: "introduction-container",
            img { src: MYICON_WEBP, class: "intro-icon" }
            div {
                class: "intro-text",
                h3 {"IWA" }
                p { "職業：エンジニア" }
                p { "趣味：Rust" }
                p { "一言：よろしくお願いします！" }
            }
        }
    }
}