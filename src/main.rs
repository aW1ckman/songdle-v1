// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{Blog, Home, Navbar, MainPage};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;
// Define backend functions
mod backend;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Blog { id: i32 },
        // Songdle route
        #[route("/songdle")]
        MainPage {},

}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");


fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::LaunchBuilder::new()
    .with_context( server_only! {
        #[cfg(feature = "server")]
        crate::s_context::AppState::new()
    })
    .launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}

#[cfg(feature = "server")]
mod s_context {
    use std::sync::Arc;

    #[derive(Clone, Debug)]
    pub struct AppState {
        pub oauth: String,
        pub db_client: Arc<aws_sdk_dynamodb::Client>,
    }

    fn get_token() -> String {
        let file = std::fs::File::open("etc/secret.json");
        match file {
            Ok(file) => {
                let data: serde_json::Value = serde_json::from_reader(file).unwrap_or_default();
                let p = data.get("oauth").unwrap().as_str().unwrap().to_string();

                println!("{}", p);
                p
            },
            Err(e) => {
                println!("Errors {e:?}");
                String::new()
            },
        }

    }

    fn create_dynamo_client() -> Arc<aws_sdk_dynamodb::Client> {
        use aws_config::meta::region::RegionProviderChain;
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::Client;
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = rt.block_on(async {
            let reigon_provider = RegionProviderChain::default_provider().or_else("eu-west-2");

            let config = aws_config::defaults(BehaviorVersion::latest())
            .region(reigon_provider)
            .load()
            .await;

            Client::new(&config)
        });
        println!("Client created");
        Arc::new(client)
    }

    impl AppState {
        pub fn new() -> Self {
            AppState {
                oauth: get_token(),
                db_client: create_dynamo_client()
            }
        }
    }

}