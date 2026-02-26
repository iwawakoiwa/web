use dioxus::prelude::*;

const FOOTER_CSS: Asset = asset!("/assets/styling/footer.css");

#[component]
pub fn Footer_component() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FOOTER_CSS }
        footer { class: "footer",
            p { "© 2026 IWA | CC BY 4.0" }
            p { "This site is independently created and is not affiliated with any company or organization." }
        }
    }
}