use dioxus::prelude::*;
use dioxus_material::{use_theme, Icon, IconKind, Chip};
use lookbook::{preview, Json, LookBook};

#[component]
fn CrateItem<'a>(
    cx: Scope<'a>,
    name: &'a str,
    version: &'a str,
    description: &'a str,
    total_downloads: u32,
    recent_downloads: u32,
    last_update: &'a str,
    links: &'a [&'a str],
) -> Element<'a> {
    let _theme = use_theme(cx);

    render!(
        li {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            width: "100%",
            max_width: "600px",
            list_style: "none",
            div {
                div { display: "flex", flex_direction: "row", gap: "10px",
                    span { font_weight: 600, name }
                    span { version }
                }
                p { description }
                ul { display: "flex", flex_direction: "row", gap: "10px", list_style: "none", margin: 0, padding: 0,
                    links.iter().map(|link| render!(Chip { onclick: |_| {}, "{link}" }))
                }
            }
            div { display: "flex", flex_direction: "column", gap: "5px",
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
        div { display: "flex", flex_direction: "row", align_items: "center", gap: "5px",
            Icon { kind: *icon }
            span { children }
        }
    )
}

/// Text fields let users enter text into a UI.
#[preview]
fn CrateItemPreview<'a>(
    cx: Scope,
    /// Label for the text field.
    #[lookbook(default = "tokio")]
    name: &'a str,

    /// Semver version of the crate.
    #[lookbook(default = "v0.1.0")]
    version: &'a str,

    /// Description of the crate.
    #[lookbook(default = "Hello world!")]
    description: &'a str,

    /// Total downloads the crate has received.
    #[lookbook(default = 25_032)]
    total_downloads: Json<u32>,

    /// Recent downloads the crate has received.
    #[lookbook(default = 164)]
    recent_downloads: Json<u32>,

    /// Last date the crate was updated.
    #[lookbook(default = "2 days ago")]
    last_update: &'a str,

    /// Links in the crate's `Cargo.toml`.
    #[lookbook(default = vec![String::from("Homepage"), String::from("Documentation")])]
    links: Json<Vec<String>>,
) -> Element<'a> {
    render!(
        CrateItem {
            name: name,
            version: version,
            description: description,
            total_downloads: total_downloads.0,
            recent_downloads: recent_downloads.0,
            last_update: last_update,
            links: &**cx
                .bump()
                .alloc(links.0.iter().cloned().map(|s| &**cx.bump().alloc(s)).collect::<Vec<_>>())
        }
    )
}

#[component]
fn Home(cx: Scope) -> Element {
    render!( h1 { "Home" } )
}

fn app(cx: Scope) -> Element {
    render!( LookBook { home: Home, previews: [CrateItemPreview] } )
}

fn main() {
    dioxus_web::launch(app);
}
