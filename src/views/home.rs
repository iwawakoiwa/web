use dioxus::{document::MetaProps, prelude::*};
use crate::components::{Footer_component, Slideshow, other_introduction};

const HOME_CSS: Asset = asset!("/assets/styling/home.css");
//const BACKIMAGE_WEBP: Asset = asset!("/assets/image/backimage.webp");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        //img { src: BACKIMAGE_WEBP, id: "backimarge"}
        div { 
            class: "main_text",
            div { 
                class:"titles", 
                h1 {"Welcome to the IWA page"}
                h2 {"IWAのホームページへようこそ"  }
                h4 {"これがホームページ、最も完璧です" }
                p {"A personal site about technology, projects, and ideas."}
                p {"技術やプロジェクト、アイデアについて発信する個人サイトです。"}
            }
            Slideshow {}  // これだけ！
         }
        div {
            class:"Second_box" ,
            h2 { "このサイトの構成" }
            ul { 
                li { "Home - サイトの入口" }
                li { "About - サイトの説明" }
                li { "Blog - 技術記事・制作記録（準備中）" }         
             }
         }
        Footer_component {}
    }
}
