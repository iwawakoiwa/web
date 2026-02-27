use dioxus::prelude::*;
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
                p {"これがホームページ、最も完璧です" }
            }
            Slideshow {}  // これだけ！
         }
        div {
            class:"Second_box" ,
            h2 { "このサイトの構成" }
            ul { 
                li { "Home - 今の場所" }
                li { "About - サイトの説明" }
                li { "Blog - Coming soon" }                
             }
         }
        Footer_component {}
    }
}
