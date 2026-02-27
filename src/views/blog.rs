use crate::{Route, components::Footer_component};
use dioxus::{html::meta::content, prelude::*};
use crate::views::markdown::to_html;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    let mds = match id {
        1 => include_str!("../../assets/markdown/1.md"),
        2 => include_str!("../../assets/markdown/2.md"),        
        3 => include_str!("../../assets/markdown/3.md"),
        _=> include_str!("../../assets/markdown/not_found.md"),       
    };

    let html = to_html(mds);

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            class: "blog",
            p { "This is blog #{id}!" }
            div { dangerous_inner_html: "{html}" }
            div {
                class:"transition",
                a {
                    Link {
                        to: Route::Blog { id: id - 1 },
                        "Previous"
                    }
                }
                span { " <---> " }
                a {  
                    Link {
                        to: Route::Blog { id: id + 1 },
                        "Next"
                    }
                }
            }
            Footer_component {  }
        }
    }
}
