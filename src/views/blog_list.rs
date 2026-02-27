use dioxus::prelude::*;
use crate::{Route, components::Footer_component};

#[component]
pub fn BlogList() -> Element {
    let articles = vec![
        (1, "最初の記事"),
        (2, "2つ目の記事"),
        (3, "3つ目の記事"),
    ];
    const BLOGLIST_CSS: Asset = asset!("assets/styling/bloglist.css");

    rsx! {
        document::Link { rel: "stylesheet", href: BLOGLIST_CSS }        
        div {  
            class:"bloglist",
            h1 { "ブログ一覧" }
            ul {
                for (id, title) in articles {
                    div {
                        class:"linkbox",                        
                        li {
                            a {
                                Link {
                                    to: Route::Blog { id },
                                    "{title}"
                                }
                            }
                        }
                    }
                }
            }
        }
        Footer_component {  }
    }
}