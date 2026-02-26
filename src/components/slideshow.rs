use dioxus::prelude::*;

const IMAGE1_WEBP: Asset = asset!("/assets/image/image1.webp");
const IMAGE2_WEBP: Asset = asset!("/assets/image/image2.webp");
const IMAGE3_WEBP: Asset = asset!("/assets/image/image3.webp");
const IMAGE4_WEBP: Asset = asset!("/assets/image/image4.webp");
const IMAGE5_WEBP: Asset = asset!("/assets/image/image5.webp");
const IMAGE6_WEBP: Asset = asset!("/assets/image/image6.webp");
const SLIDESHOW_CSS: Asset = asset!("/assets/styling/slideshow.css");

#[component]
pub fn Slideshow() -> Element {  // 大文字始まりに
    let mut current = use_signal(|| 0usize);
    let images = vec![IMAGE1_WEBP, IMAGE2_WEBP, IMAGE3_WEBP, IMAGE4_WEBP, IMAGE5_WEBP,IMAGE6_WEBP];

    let len = images.len(); // define it before use_effect
    let mut is_playing = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(3000).await;

                if is_playing() {
                    current.set((current() + 1) % len);
                }
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: SLIDESHOW_CSS }
        div { class: "slideshow",
            img { src: images[current()], class: "slide-image" }  // ima → img
            div { class: "slide-buttons",  // クラス名修正
                button {
                    onclick: move |_| {  // oncliick → onclick
                        if current() > 0 { current -= 1; }  // () → {}
                    }, "←"
                }
                button {
                    onclick: move |_| {  // oncliick → onclick
                        if current() < images.len() - 1 { current += 1; }  // > → 
                    }, "→"
                }
                button {
                    onclick: move |_| is_playing.set(!is_playing()),
                    if is_playing() { "⏸" } else { "▶" }                       
                    
                }
            }
        }
    }
}