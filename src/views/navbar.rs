use crate::Route;
use dioxus::{html::embed::width, prelude::*};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const MYICON_WEBP: Asset = asset!("assets/image/myicon.webp");
const GITHUBICON_WEBP: Asset = asset!("assets/image/GitHub_Invertocat_White.svg");
/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div {
            id: "navbar",
            img { src: MYICON_WEBP, id: "myicon",}
            a { 
                href: "https://dioxuslabs.com/learn/0.7/",
                img { src: GITHUBICON_WEBP,id:"myicon",style:"margin-top:0px; margin-left: 10px;" }
            }
            Link {
                to: Route::Home {},
                "IWAのサーバーへようこそ"
            }
            Link {
                to: Route::BlogList  {},
                "Blog"
            }
            Link {
                to: Route::Introduction {},
                "self-introduction"
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
