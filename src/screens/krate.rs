use crate::{api, components::Krate, KrateTab, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_signals::use_signal;

#[component]
pub fn KrateScreen(cx: Scope, name: String, tab: KrateTab) -> Element {
    let navigator = use_navigator(cx);
    let crates = use_signal(cx, || None);

    let name_clone = name.clone();
    use_effect(cx, (), |_| async move {
        let data = api::get_crate(&name_clone).await.unwrap();
        crates.set(Some(data));
    });

    let crates_ref = crates.read();

    let selected = match tab {
        KrateTab::Readme => 0,
        KrateTab::Versions => 1
    };

    if let Some(krate) = &*crates_ref {
        let description = krate.krate.description.as_deref().unwrap_or_default();

        render!(
            Krate {
                name: "{name}",
                description: "{description}",
                version: "{krate.krate.newest_version}",
                versions: cx.bump().alloc(krate.versions.clone()),
                selected: selected,
                onselect: |idx| {
                    let tab = match idx {
                        0 => KrateTab::Readme,
                        1 => KrateTab::Versions,
                        _ => todo!(),
                    };
                    navigator
                        .push(Route::KrateScreen {
                            name: name.clone(),
                            tab,
                        });
                }
            }
        )
    } else {
        None
    }
}
