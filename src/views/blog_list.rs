use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn BlogList() -> Element {
    rsx! {
        h1 { "ブログ一覧" }
        Link { to: Route::Blog { id: 0 }, "記事1" }
        Link { to: Route::Blog { id: 1 }, "記事2" }
    }
}