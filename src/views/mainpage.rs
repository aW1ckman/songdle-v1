
use dioxus::prelude::*;

const MAINPAGE_CSS: Asset = asset!("/assets/styling/mainpage.css");

/// Songdle page
#[component]
pub fn MainPage() -> Element {

    rsx! {
        
        document::Link { rel: "stylesheet", href: MAINPAGE_CSS }

        div {
            id: "mainpage",

            // Content
            h1 { "This is mainpage!" }
        }
    }
}
