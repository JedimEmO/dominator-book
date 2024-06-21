use uuid::Uuid;
use anyhow::Result;
use dominator::{clone, Dom, events, html};
use futures_signals::signal::Mutable;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug)]
struct ItemInfo;

// ANCHOR: trait
pub trait ItemRepository {
    async fn get_item_info(&self, item_id: Uuid) -> Result<ItemInfo>;
}
// ANCHOR_END: trait

// ANCHOR: renderer
pub fn item_info_view(info: &Option<ItemInfo>) -> Dom {
    if let Some(info) = info {
        html!("span", { .text(format!("Item info: {:?}", info).as_str())})
    } else {
        html!("span", { .text("Loading item info, please wait!") })
    }
}
// ANCHOR_END: renderer

// ANCHOR: component
pub fn item_info(repository: &'static impl ItemRepository, item_id: Uuid) -> Dom {
    let item_info = Mutable::new(None);

    html!("div", {
        .future(clone!(item_info => async move {
            let info = repository.get_item_info(item_id).await.unwrap();
            item_info.set(Some(info));
        }))
        .child_signal(item_info.signal_ref(|info| {
            Some(item_info_view(info))
        }))
    })
}
// ANCHOR_END: component

#[rustfmt::skip]
fn spawn_local_example() -> Dom {
// ANCHOR: spawn_local
html!("button", {
    .event(|_: events::Click| {
        spawn_local(async move {
            async move {
                info!("Yay, I was polled!")
            }.await
        })
    })
})
// ANCHOR_END: spawn_local
}