use dominator::routing::{go_to_url, url};
use dominator::traits::AsStr;
use dominator::{html, link, Dom};
use futures_signals::signal::{Signal, SignalExt};
use regex::Regex;
use wasm_bindgen::UnwrapThrowExt;

#[rustfmt::skip]
async fn basic_url() {
// ANCHOR: route_url
// Print the current value
let url = dominator::routing::url();
info!("You are currently at: {}", url.get_cloned());
    
// Print every new url
url.signal_cloned().for_each(|new_url| async move {
    info!("You are now at: {}", new_url)
}).await;
// ANCHOR_END: route_url
    
// ANCHOR: change_url
dominator::routing::go_to_url("#/new-url");
// ANCHOR_END: change_url
}

// ANCHOR: hello_world_url
fn hello_world() -> Dom {
    let child = url()
        .signal_ref(|current_route| {
            web_sys::Url::new(current_route.as_str()).expect_throw("Invalid url")
        })
        .map(
            |current_route: web_sys::Url| match current_route.hash().as_str() {
                "#/hello" => Some(html!("span", { .text("Hello, world!")})),
                _ => Some(html!("span", { .text("Unknown route, sorry!") })),
            },
        );

    html!("div", {
        .child(html!("h1", { .text("Simple routing example")}))
        .child_signal(child)
    })
}
// ANCHOR_END: hello_world_url

// ANCHOR: route_enum
enum ShopRoutes {
    LandingPage,
    Shop { display_item_id: Option<String> },
}
// ANCHOR_END: route_enum

// ANCHOR: route_enum_render
fn shop_application(route: impl Signal<Item = ShopRoutes>) -> impl Signal<Item = Option<Dom>> {
    route.map(|new_route| match new_route {
        ShopRoutes::Shop { display_item_id } => match display_item_id {
            Some(item_id) => Some(
                html!("span", { .text(format!("Displaying item with id {}", item_id).as_str()) }),
            ),
            None => Some(html!("span", { .text("Displaying all items") })),
        },
        _ => Some(html!("Welcome to our shop!")),
    })
}
// ANCHOR_END: route_enum_render

// ANCHOR: route_enum_methods
impl ShopRoutes {
    pub fn signal() -> impl Signal<Item = Self> {
        let shop_item_regex = Regex::new(r"#/shop/(?<item_id>[0-9]+)").unwrap();

        url().signal_ref(move |new_route_path| {
            let url = web_sys::Url::new(new_route_path.as_str()).expect_throw("Invalid url");
            let hash = url.hash();

            if let Some(captured_item_id) = shop_item_regex
                .captures(hash.as_str())
                .map(|captures| captures["item_id"].to_string())
            {
                return Self::Shop {
                    display_item_id: Some(captured_item_id),
                };
            }

            match hash.as_str() {
                "#/shop" => ShopRoutes::Shop {
                    display_item_id: None,
                },
                _ => ShopRoutes::LandingPage,
            }
        })
    }

    pub fn goto(route: Self) {
        go_to_url(route.to_url().as_str());
    }

    pub fn to_url(&self) -> String {
        match self {
            ShopRoutes::LandingPage => "#/landing".to_string(),
            ShopRoutes::Shop { display_item_id } => match display_item_id {
                Some(item_id) => format!("#/shop/{}", item_id),
                None => "#/shop".to_string(),
            },
        }
    }
}
// ANCHOR_END: route_enum_methods

// ANCHOR: route_enum_full
fn route_enum_example() -> Dom {
    let route_signal = ShopRoutes::signal();

    html!("div", {
        .child(html!("h1", { .text("Route enum example") }))
        .child(html!("div", {
            .child(html!("span", {
                .text("Landing Page")
                .event(|_: dominator::events::Click| {
                    ShopRoutes::goto(ShopRoutes::LandingPage)
                })
            }))
            .child(html!("span", {
                .text("Shop")
                .event(|_: dominator::events::Click| {
                    ShopRoutes::goto(ShopRoutes::Shop { display_item_id: None })
                })
            }))
            .child(link!(
                ShopRoutes::Shop { display_item_id: Some("1234".to_string())}.to_url(),
                {
                    .text("Daily offer!")
                }))
        }))
        .child(html!("div", {
            .class("main-view")
            .child_signal(shop_application(route_signal))
        }))
    })
}
// ANCHOR_END: route_enum_full
