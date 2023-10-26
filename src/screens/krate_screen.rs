use crate::{api, components::Krate};
use dioxus::prelude::*;
use dioxus_signals::use_signal;

#[component]
pub fn KrateScreen(cx: Scope, name: String) -> Element {
    let crates = use_signal(cx, || None);

    let name_clone = name.clone();
    use_effect(cx, (), |_| async move {
        let data = api::get_crate(&name_clone).await.unwrap();
        crates.set(Some(data));
    });

    let crates_ref = crates.read();

    if let Some(krate) = &*crates_ref {
        let description = krate.krate.description.as_deref().unwrap_or_default();

        render!(Krate {
            name: "{name}",
            description: "{description}",
            version: "{krate.krate.newest_version}"
        })
    } else {
        None
    }
}
