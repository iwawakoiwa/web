use crate::components::Hero;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");
const BACKIMAGE_WEBP: Asset = asset!("/assets/image/backimage.webp");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        img { src: BACKIMAGE_WEBP, id: "backimarge"}
        
        h1 {"iwaのホームページ"}

    }
}
