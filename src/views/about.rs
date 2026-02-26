use dioxus::prelude::*;

const ABOUT_CSS: Asset = asset!("assets/styling/about.css");
const ICON_SVG: Asset = asset!("assets/image/GitHub_Invertocat_White.svg");

#[component]
pub fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ABOUT_CSS }
        div { 
            div {
                class:"about",            
                h1 { "このサイトで使用されている技術" }
                p { "このサイトはRustのみで作られています。フロントエンドはDioxusというフレームワークを使いWebAssemblyにコンパイルされ、バックエンドはAxumで動いています。" }
                br {}
                p { class:"Annotation","githubのリポジトリがある場合があるならリンクで飛べます"}
                h2 {"フロントエンド"}
                ul { 
                    a {
                        href: "https://github.com/iwawakoiwa/web",
                        target: "_blank",  // 新しいタブで開く
                        "Rust + Dioxus (WebAssembly)"
                    }
                    p { "CSS" }
                 }
                h2 { "バックエンド" }
                ul { 
                    a {
                        href: "https://github.com/iwawakoiwa/web_rust",
                        target: "_blank",  // 新しいタブで開く
                        "RUST + Axum + Tokio(非同期処理)"
                    }
                 }
                h2 { "インフラ" }
                ul {
                    p { "Docker + Debian" }
                    p { "セルフホスティング" }
                }
            }
         }
    }
}