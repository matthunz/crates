use dioxus::prelude::*;
use dioxus_material::{use_theme, Chip, Icon, IconKind};

/// Item in a list of crates.
#[component]
pub fn KrateItem<'a>(
    cx: Scope<'a>,

    /// Name of the crate.
    name: &'a str,

    /// Semver version of the crate.
    version: &'a str,

    /// Description of the crate.
    description: &'a str,

    /// Total downloads the crate has received.
    total_downloads: u32,

    /// Recent downloads the crate has received.
    recent_downloads: u32,

    /// Last date the crate was updated.
    last_update: &'a str,

    /// Links in the crate's `Cargo.toml`.
    links: &'a [&'a str],

    /// Handler for click events.
    onclick: EventHandler<'a>,
) -> Element<'a> {
    let _theme = use_theme(cx);

    render!(
        li {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            max_width: "800px",
            list_style: "none",
            padding_bottom: "20px",
            border_bottom: "2px solid #eee",
            div { display: "flex", flex_direction: "row", align_items: "center", gap: "10px", margin_bottom: "10px",
                span {
                    font_weight: 600,
                    max_width: "200px",
                    overflow: "hidden",
                    text_overflow: "ellipsis",
                    cursor: "pointer",
                    onclick: |_| onclick.call(()),
                    name
                }
                Chip { onclick: |_| onclick.call(()), version }
            }
            p { margin: 0, padding: 0, description }
            ul { display: "flex", flex_direction: "row", gap: "10px", list_style: "none", margin: 0, padding: 0,
                links.iter().map(|link| render!(Chip { onclick: |_| {}, "{link}" }))
            }
            div { display: "flex", flex_direction: "row", align_items: "center", margin_top: "20px",
                Statistic { icon: IconKind::Download, "{total_downloads}" }
                Statistic { icon: IconKind::History, "{recent_downloads}" }
                Statistic { icon: IconKind::Schedule, "{last_update}" }
            }
        }
    )
}

#[component]
fn Statistic<'a>(cx: Scope<'a>, icon: IconKind, children: Element<'a>) -> Element<'a> {
    render!(
        div { flex: 1, display: "flex", flex_direction: "row", align_items: "center", gap: "5px",
            Icon { kind: *icon }
            span { children }
        }
    )
}

#[cfg(feature = "lookbook")]
/// Item in a list of crates.
#[lookbook::preview]
pub fn KrateItemPreview<'a>(
    cx: Scope,
    /// Name of the crate.
    #[lookbook(default = "tokio")]
    name: &'a str,

    /// Semver version of the crate.
    #[lookbook(default = "v0.1.0")]
    version: &'a str,

    /// Description of the crate.
    #[lookbook(default = "Hello world!")]
    description: &'a str,

    /// Total downloads the crate has received.
    #[lookbook(default = 25032)]
    total_downloads: lookbook::Json<u32>,

    /// Recent downloads the crate has received.
    #[lookbook(default = 164)]
    recent_downloads: lookbook::Json<u32>,

    /// Last date the crate was updated.
    #[lookbook(default = "2 days ago")]
    last_update: &'a str,

    /// Links in the crate's `Cargo.toml`.
    #[lookbook(default = vec![String::from("Homepage"), String::from("Documentation")])]
    links: lookbook::Json<Vec<String>>,
) -> Element<'a> {
    render!(KrateItem {
        name: name,
        version: version,
        description: description,
        total_downloads: total_downloads.0,
        recent_downloads: recent_downloads.0,
        last_update: last_update,
        links: &**cx.bump().alloc(
            links
                .0
                .iter()
                .cloned()
                .map(|s| &**cx.bump().alloc(s))
                .collect::<Vec<_>>()
        ),
        onclick: |_| {}
    })
}
