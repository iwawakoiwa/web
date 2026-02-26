use dioxus::prelude::*;
use crate::Route;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");
const MYICON_WEBP: Asset = asset!("assets/image/myicon.webp");
//const BACKIMAGE_WEBP: Asset = asset!("/assets/image/backimage.webp");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        //img { src: BACKIMAGE_WEBP, id: "backimarge"}
        div { 
            id: "main_text",
            h1 {"Welcome to the IWA page"}
            h2 {"IWAのホームページへようこそ"  }
         }
        //  div {
        //     id:"buttons" ,
        //     div {                   
        //         Link { to: Route::Home {},class:"button", "Self introduction" }
        //         br {}    
        //         Link { to: Route::Blog {id:0},class:"button",  "Awesome Dioxus" }
        //     }
        //   }
        div {
            class: "introduction-container",
            img { src: MYICON_WEBP, class: "intro-icon" }
            div {
                class: "intro-text",
                h2 { "IWA" }
                p { "職業：エンジニア" }
                p { "趣味：Rust" }
                p { "一言：よろしくお願いします！" }
            }
        }

    }
}
