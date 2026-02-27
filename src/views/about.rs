use dioxus::prelude::*;

use crate::components::{Footer_component, other_introduction};

const ABOUT_CSS: Asset = asset!("assets/styling/about.css");

#[component]
pub fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ABOUT_CSS }
        div { 
            div {
                class:"about",            
                h1 { "このサイトの概要" }
                div {
                    h2 { "このサイト作成者" }
                    other_introduction {}
                    h3 {"作成目的"}
                    p {"このサイトは、自分の技術的な実験や学習の記録として作成しました。
                        インターネット上の自分の家として、自由に更新していきます。"}
                }
                div {
                    style:"margin-top: 10px;",
                    h2{"使用されている技術"}
                    p { "このサイトはRustでフルスクラッチ開発しています。
                        フロントエンドからバックエンド、インフラまで自分で構築しています。" }
                    br {}
                    p { class:"Annotation","githubのリポジトリがある場合があるならリンクで飛べます"}
                    h3 {"フロントエンド"}
                    ul { 
                        a {
                            href: "https://github.com/iwawakoiwa/web",
                            target: "_blank",  // 新しいタブで開く
                            "Rust + Dioxus (WebAssembly)"
                        }
                        p { "CSS" }
                     }
                    h3 { "バックエンド" }
                    ul { 
                        a {
                            href: "https://github.com/iwawakoiwa/web_rust",
                            target: "_blank",  // 新しいタブで開く
                            "RUST + Axum + Tokio(非同期処理)"
                        }
                     }
                    h3 { "インフラ" }
                    ul {
                        p { "Docker + Debian" }
                        p { "セルフホスティング" }
                    }
                }
            }
        
         }
        Footer_component {  }   
    }
}