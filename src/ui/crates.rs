use crate::{api, ui::CrateItem};
use dioxus::prelude::*;
use dioxus_signals::use_signal;

#[component]
pub fn Crates(cx: Scope) -> Element {
    let crates = use_signal(cx, || None);

    use_effect(cx, (), |_| async move {
        let data = api::crates(1, 50).await.unwrap();
        crates.set(Some(data));
    });

    render!(
        ul {
            if let Some(crates) = &*crates.read() {
                render!(crates.iter().map(|krate| render!(CrateItem {
                    name: "{krate.name}",
                    version: "{krate.newest_version}",
                    description: "{krate.description.clone().unwrap_or_default()}",
                    total_downloads: krate.downloads,
                    recent_downloads: krate.recent_downloads,
                    last_update: "{krate.updated_at}",
                    links: &[]
                })))
            } else {
                None
            }
        }
    )
}
